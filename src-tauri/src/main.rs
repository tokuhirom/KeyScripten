mod send;
mod grab;
mod event;
mod app_config;
mod keycode;
mod hotkey;
mod js;
mod js_builtin;

use std::{fs, thread};

use std::str::FromStr;

use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventFlags, CGKeyCode};

use chrono::Local;
use log::LevelFilter;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use crate::app_config::AppConfig;

use crate::grab::run_handler;
use crate::hotkey::HotKey;

const APP_NAME: &str = "onemoretime";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> anyhow::Result<()> {
    let app_config = AppConfig::load()?;

    let level_filter = match LevelFilter::from_str(app_config.log_level.as_str()) {
        Ok(level) => {level}
        Err(err) => {
            log::error!("Unknown log level in configuration: {:?},{:?}", app_config.log_level, err);
            LevelFilter::Info
        }
    };

    let log_path = dirs::data_dir().unwrap()
        .join(APP_NAME)
        .join("onemoretime.log");
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
        .level(level_filter)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;

    log::info!("Default log level is `{}`", level_filter);
    log::info!("Shortcut key is: `{}`", app_config.repeat_shortcut);

    let _hotkey = HotKey::from_str(app_config.repeat_shortcut.as_str())?;

    thread::spawn(move || {
        log::debug!("Starting handler thread: {:?}", thread::current().id());
        run_handler();
    });

    log::debug!("Creating menu object");

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);

    let tray = SystemTray::new()
        .with_menu(tray_menu);

    log::debug!("Building tauri");

    if let Err(err) = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

            #[allow(clippy::single_match)]
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!()) {
        log::error!("Cannot start tauri app: {:?}", err);
        return Err(anyhow!("Cannot start tauri app: {:?}", err));
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct KeyState {
    code: CGKeyCode,
    flags: CGEventFlags,
}
