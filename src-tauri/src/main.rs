use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::{fs, thread};

use std::str::FromStr;
use std::sync::{mpsc, Arc, RwLock};

use anyhow::anyhow;

use chrono::{Local, SecondsFormat};
use keyscripten_core::app_config::{AppConfig, PluginConfig};
use keyscripten_core::event::Event;
use keyscripten_core::grab::{grab_run, grab_setup};
use keyscripten_core::js::{ConfigSchema, ConfigSchemaList, JS};
use keyscripten_core::js_console::TimedLogMessage;
use keyscripten_core::js_operation::JsOperation;
use keyscripten_core::plugin::Plugins;
use lazy_static::lazy_static;
use log::{LevelFilter, Record};
use tauri::api::dialog;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    WindowBuilder, Wry,
};

const APP_NAME: &str = "keyscripten";

static mut LOG_LEVEL: RwLock<LevelFilter> = RwLock::new(LevelFilter::Info);

lazy_static! {
    static ref VEC_DEQUE: Arc<RwLock<VecDeque<Event>>> = Arc::new(RwLock::new(VecDeque::new()));
}
lazy_static! {
    static ref LOG_BUFFER: RwLock<VecDeque<String>> = RwLock::new(VecDeque::new());
}

fn build_js<'a>() -> Result<JS<'a>, String> {
    let plugins = Plugins::new().map_err(|err| format!("Plugins::new: {:?}", err))?;
    let mut js = JS::new(None, None, Some(plugins)).map_err(|err| format!("{:?}", err))?;
    js.load_user_scripts()
        .map_err(|err| format!("load_user_scripts: {:?}", err))?;
    Ok(js)
}

fn get_filename_by_plugin_id(plugin_id: String) -> Result<String, String> {
    let js = build_js().map_err(|err| format!("Cannot build js: {:?}", err))?;
    if let Some(filename) = js.get_filename_by_plugin_id(&plugin_id) {
        Ok(filename)
    } else {
        log::error!("There's no plugin file found for {}", plugin_id);
        Err(format!("There's no plugin file found for {}", plugin_id))
    }
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
    let plugins = Plugins::new().map_err(|err| format!("Cannot list plugin: {:?}", err))?;
    plugins
        .list()
        .map_err(|err| format!("Cannot add plugin: {:?}", err))
}

#[tauri::command]
fn get_plugin_filename(plugin_id: String) -> Result<String, String> {
    let filename = get_filename_by_plugin_id(plugin_id)?;
    Ok(filename)
}

#[tauri::command]
fn read_plugin_code(plugin_id: String) -> Result<String, String> {
    let filename = get_filename_by_plugin_id(plugin_id)?;

    let plugins = Plugins::new().map_err(|err| format!("Cannot read plugin: {:?}", err))?;
    let plugin_snippet = plugins
        .read(filename)
        .map_err(|err| format!("Cannot read plugin: {:?}", err))?;
    Ok(plugin_snippet.src)
}

#[tauri::command]
fn write_plugin_code(plugin_id: String, code: String) -> Result<(), String> {
    log::info!("tauri::command: write_plugin_code: {}", plugin_id);

    let filename = get_filename_by_plugin_id(plugin_id)?;
    let plugins = Plugins::new().map_err(|err| format!("Cannot write plugin: {:?}", err))?;
    plugins
        .write(filename, code)
        .map_err(|err| format!("Cannot write plugin: {:?}", err))
}

#[tauri::command]
fn delete_plugin(plugin_id: String) -> Result<(), String> {
    log::info!("tauri::command: delete_plugin: {}", plugin_id);

    let filename = get_filename_by_plugin_id(plugin_id)?;
    let plugins =
        Plugins::new().map_err(|err| format!("Cannot construct Plugins instance: {:?}", err))?;
    plugins
        .delete(filename)
        .map_err(|err| format!("Cannot delete plugin: {:?}", err))
}

#[tauri::command]
fn read_logs() -> Result<Vec<String>, String> {
    log::debug!("tauri::command: read_logs");

    let buffer = LOG_BUFFER
        .read()
        .map_err(|err| format!("Cannot get lock: {:?}", err))?;
    Ok(buffer.iter().cloned().collect())
}

#[tauri::command]
fn read_console_logs() -> Result<Vec<TimedLogMessage>, String> {
    log::debug!("tauri::command: read_console_logs");

    let buffer = keyscripten_core::js_console::get_console_logs();
    Ok(buffer)
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

fn build_log_path() -> PathBuf {
    dirs::data_dir()
        .unwrap()
        .join(APP_NAME)
        .join("keyscripten.log.")
}

fn logger() -> anyhow::Result<()> {
    let log_path = dirs::data_dir()
        .unwrap()
        .join(APP_NAME)
        .join("keyscripten.log");
    log::info!("Logging file is output to {:?}", log_path);
    fs::create_dir_all(log_path.parent().unwrap())
        .map_err(|err| anyhow!("Cannot create {:?}: {:?}", log_path, err))?;

    let log_prefix = build_log_path();
    let log_prefix = log_prefix.to_str().unwrap();
    log::info!("Log file prefix is: {:?}", log_prefix);

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {} {}] {}",
                Local::now().to_rfc3339_opts(SecondsFormat::Secs, false),
                thread_id::get(),
                record.level(),
                record.target(),
                message
            ))
        })
        .filter(|metadata| unsafe { metadata.level() <= *LOG_LEVEL.read().unwrap() })
        .chain(std::io::stdout())
        .chain(fern::DateBased::new(log_prefix, "%Y-%m-%d"))
        .chain(fern::log_file(log_path)?)
        .chain(fern::Output::call(move |record: &Record| {
            let mut buffer = LOG_BUFFER.write().unwrap();
            if buffer.len() >= 40 {
                buffer.pop_front();
            }
            buffer.push_back(format!("{}", record.args()));
        }))
        .apply()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    logger()?;

    let app_config = AppConfig::load()?;
    set_log_level_by_config(&app_config);

    let (js_operation_tx, js_operation_rx) = mpsc::channel::<JsOperation>();
    let (setup_tx, setup_rx) = mpsc::channel::<anyhow::Result<()>>();

    thread::spawn(move || {
        log::debug!("Starting handler thread: {:?}", thread::current().id());
        let plugins = Plugins::new().expect("Cannot load plugins");
        let mut js = JS::new(
            Some(js_operation_rx),
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
            app.listen_global("js-operation", move |event| {
                // update-config
                log::info!("js-operation: {:?}", event);
                let js_operation: JsOperation = serde_json::from_str(event.payload().unwrap())
                    .expect("Deserialize js-operation");
                js_operation_tx.send(js_operation)
                    .expect("Send message");
            });

            log::info!("Waiting CGEventTapCreate");
            let setup_result = setup_rx.recv().expect("Setup message received");
            if let Err(err) = setup_result {
                log::error!("Cannot run handler: {:?}", err);
                dialog::message::<Wry>(
                    None,
                    "KeyScripten",
                    format!(
                        "Cannot setup CGEventTapCreate: {:?}\nPlease read the document for more details: https://github.com/tokuhirom/KeyScripten/blob/main/README.md",
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
                        let window_label = "config-window".to_string();
                        if let Some(window) = app.get_window(&window_label) {
                            // If it exists, focus the existing window
                            if let Err(err) = window.show() {
                                log::error!("Cannot show configuration window: {:?}", err);
                            }
                            if let Err(err) = window.set_focus() {
                                log::error!("Cannot focus on existing configuration window: {:?}", err);
                            }
                        } else {
                            match WindowBuilder::new(
                                app,
                                "config-window".to_string(),
                                tauri::WindowUrl::App("index.html".into()),
                            ).build() {
                                Ok(window) => {
                                    if let Err(err) = window.set_title("KeyScripten") {
                                        log::error!("Cannot set window title: {:?}", err);
                                    }
                                }
                                Err(err) => {
                                    log::error!("Cannot open configuration window: {:?}", err);
                                }
                            }
                        }
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
            read_plugin_code,
            write_plugin_code,
            delete_plugin,
            read_logs,
            read_console_logs,
            get_plugin_filename,
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
