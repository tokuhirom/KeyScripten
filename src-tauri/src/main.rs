use std::collections::{HashMap, VecDeque};
use std::{fs, thread};

use std::str::FromStr;
use std::sync::{mpsc, Arc, RwLock};

use anyhow::anyhow;

use chrono::Local;
use codekeys_core::app_config::{AppConfig, PluginConfig};
use codekeys_core::event::Event;
use codekeys_core::grab::{grab_run, grab_setup};
use codekeys_core::js::{ConfigSchema, ConfigSchemaList, JS};
use codekeys_core::plugin::Plugins;
use lazy_static::lazy_static;
use log::LevelFilter;
use tauri::api::dialog;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    WindowBuilder, Wry,
};

const APP_NAME: &str = "codekeys";

static mut LOG_LEVEL: RwLock<LevelFilter> = RwLock::new(LevelFilter::Info);

lazy_static! {
    static ref VEC_DEQUE: Arc<RwLock<VecDeque<Event>>> = Arc::new(RwLock::new(VecDeque::new()));
}

fn build_js<'a>() -> Result<JS<'a>, String> {
    let plugins = Plugins::new().map_err(|err| format!("Plugins::new: {:?}", err))?;
    let mut js = JS::new(None, None, None, Some(plugins)).map_err(|err| format!("{:?}", err))?;
    js.load_user_scripts()
        .map_err(|err| format!("load_user_scripts: {:?}", err))?;
    Ok(js)
}

#[tauri::command]
fn get_config_schema() -> Result<ConfigSchemaList, String> {
    let mut js = build_js()?;
    js.get_config_schema()
        .map_err(|err| format!("get_config_schema: {:?}", err))
}

#[tauri::command]
fn get_config_schema_for_plugin(plugin_id: String) -> Result<ConfigSchema, String> {
    let mut js = build_js()?;
    let schema_list = js.get_config_schema().map_err(|err| format!("{:?}", err))?;
    for plugin in schema_list.plugins {
        if plugin.id == plugin_id {
            return Ok(plugin);
        }
    }
    Err(format!(
        "Cannot load configuration schema for {}",
        plugin_id
    ))
}

#[tauri::command]
fn load_config() -> Result<AppConfig, String> {
    AppConfig::load().map_err(|err| format!("{:?}", err))
}

#[tauri::command]
fn save_config_for_plugin(plugin_id: String, plugin_config: PluginConfig) -> Result<(), String> {
    let mut config = AppConfig::load()
        .map_err(|err| format!("An error occurred while loading configuration: {:?}", err))?;
    config
        .plugins
        .get_or_insert(HashMap::new())
        .insert(plugin_id.clone(), plugin_config);
    config
        .save()
        .map_err(|err| format!("Cannot save configuration for {}: {:?}", plugin_id, err))?;
    Ok(())
}

#[tauri::command]
fn load_config_for_plugin(plugin_id: String) -> Result<PluginConfig, String> {
    let config = AppConfig::load()
        .map_err(|err| format!("An error occurred while loading configuration: {:?}", err))?;
    match config.plugins {
        Some(plugins) => match plugins.get(&plugin_id) {
            Some(config) => Ok((*config).clone()),
            None => Ok(PluginConfig::default()),
        },
        None => Ok(PluginConfig::default()),
    }
}

#[tauri::command]
fn update_log_level(log_level: String) -> Result<(), String> {
    let mut config = AppConfig::load()
        .map_err(|err| format!("An error occurred while loading configuration: {:?}", err))?;
    config.log_level = log_level;
    config.save().map_err(|err| format!("{:?}", err))?;
    set_log_level_by_config(&config);
    Ok(())
}

#[tauri::command]
fn get_event_log() -> Result<Vec<Event>, String> {
    let result = VEC_DEQUE
        .read()
        .map_err(|err| format!("An error occurred while getting lock: {:?}", err))?;
    Ok(result.iter().cloned().collect())
}

#[tauri::command]
fn add_plugin(plugin_id: String, name: String, description: String) -> Result<(), String> {
    let plugins = Plugins::new().map_err(|err| format!("Cannot add plugin: {:?}", err))?;
    plugins
        .add(plugin_id, name, description)
        .map_err(|err| format!("Cannot add plugin: {:?}", err))
}

#[tauri::command]
fn list_plugins() -> Result<Vec<String>, String> {
    let plugins = Plugins::new().map_err(|err| format!("Cannot add plugin: {:?}", err))?;
    plugins
        .list()
        .map_err(|err| format!("Cannot add plugin: {:?}", err))
}

fn set_log_level_by_config(app_config: &AppConfig) {
    let level_filter = match LevelFilter::from_str(app_config.log_level.as_str()) {
        Ok(level) => level,
        Err(err) => {
            log::error!(
                "Unknown log level in configuration: {:?},{:?}",
                app_config.log_level,
                err
            );
            LevelFilter::Info
        }
    };
    set_log_level(level_filter);
}
fn set_log_level(level_filter: LevelFilter) {
    unsafe {
        eprintln!("Setting log level to {:?}", level_filter);
        *LOG_LEVEL.write().unwrap() = level_filter;
        log::info!("Set log level to `{}`", level_filter);
    }
}

fn logger() -> anyhow::Result<()> {
    let log_path = dirs::data_dir()
        .unwrap()
        .join(APP_NAME)
        .join("codekeys.log");
    log::info!("Logging file is output to {:?}", log_path);
    fs::create_dir_all(log_path.parent().unwrap())
        .map_err(|err| anyhow!("Cannot create {:?}: {:?}", log_path, err))?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().to_rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .filter(|metadata| unsafe { metadata.level() <= *LOG_LEVEL.read().unwrap() })
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    logger()?;

    let app_config = AppConfig::load()?;
    set_log_level_by_config(&app_config);

    let (config_reload_tx, config_reload_rx) = mpsc::channel::<bool>();
    let (plugin_reload_tx, plugin_reload_rx) = mpsc::channel::<bool>();
    let (setup_tx, setup_rx) = mpsc::channel::<anyhow::Result<()>>();

    thread::spawn(move || {
        log::debug!("Starting handler thread: {:?}", thread::current().id());
        let plugins = Plugins::new().expect("Cannot load plugins");
        let mut js = JS::new(
            Some(config_reload_rx),
            Some(plugin_reload_rx),
            Some(Arc::clone(&VEC_DEQUE)),
            Some(plugins),
        )
        .expect("Cannot create JS instance");
        if let Err(err) = js.load_user_scripts() {
            log::error!("Cannot load plugin: {:?}", err);
        }

        let result = grab_setup(js);
        if let Err(err) = &result {
            log::error!("Cannot run handler: {:?}", err);
        }
        setup_tx.send(result).expect("Send setup message");
        grab_run();
    });

    log::debug!("Creating menu object");

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let configuration = CustomMenuItem::new("configuration".to_string(), "Configuration");
    let tray_menu = SystemTrayMenu::new()
        .add_item(configuration)
        .add_native_item(SystemTrayMenuItem::Separator) // separator
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    log::debug!("Building tauri");

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(move |app| {
            app.listen_global("update-config", move |event| {
                log::info!("update-config: {:?}", event);
                config_reload_tx.send(true).expect("Send message");
            });
            app.listen_global("reload-plugins", move |event| {
                log::info!("reload-plugins: {:?}", event);
                plugin_reload_tx.send(true).expect("Send message");
            });

            log::info!("Waiting CGEventTapCreate");
            let setup_result = setup_rx.recv().expect("Setup message received");
            if let Err(err) = setup_result {
                log::error!("Cannot run handler: {:?}", err);
                dialog::message::<Wry>(
                    None,
                    "CodeKeys",
                    format!(
                        "Cannot setup CGEventTapCreate: {:?}\nPlease read the document for more details: https://github.com/tokuhirom/CodeKeys/blob/main/README.md",
                        err
                    ),
                );
            }

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

            #[allow(clippy::single_match)]
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "configuration" => {
                        log::info!("Got configuration event");
                        if let Err(err) = WindowBuilder::new(
                            app,
                            "config-window".to_string(),
                            tauri::WindowUrl::App("index.html".into()),
                        )
                        .build()
                        {
                            log::error!("Cannot open configuration window: {:?}", err);
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_config_schema,
            load_config,
            save_config_for_plugin,
            load_config_for_plugin,
            get_config_schema_for_plugin,
            update_log_level,
            get_event_log,
            add_plugin,
            list_plugins,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });

    Ok(())
}
