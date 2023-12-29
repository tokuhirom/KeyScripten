mod send;
mod grab;
mod event;
mod sender;
mod state;
mod handler;
mod app_config;
mod keycode;
mod shortcut;

use std::{fs, thread};
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventFlags, CGKeyCode};
use chrono::Local;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use handler::Handler;
use crate::app_config::AppConfig;

use crate::grab::grab_ex;
use crate::shortcut::parse_shortcut;

const APP_NAME: &str = "onemoretime";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> anyhow::Result<()> {
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
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;

    let app_config = AppConfig::load()?;
    log::info!("Shortcut key is: `{}`", app_config.repeat_shortcut);

    let shortcut = parse_shortcut(app_config.repeat_shortcut.as_str())?;

    thread::spawn(move || {
        let mut handler = Handler::new(64, shortcut);
        if let Err(error) = grab_ex(move |event| {
            handler.callback(event)
        }) {
            println!("Error: {:?}", error)
        }
    });

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);

    let tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct KeyState {
    code: CGKeyCode,
    flags: CGEventFlags,
}
