mod send;
mod grab;
mod event;
mod sender;
mod state;
mod handler;
mod app_config;
mod keycode;
mod shortcut;
mod js;

use std::{fs, thread};
use std::arch::aarch64::vbic_s8;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventField_kCGKeyboardEventKeycode, CGEventFlags, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp, CGKeyCode, exit};
use boa_engine::{Context, js_string, JsObject, JsResult, JsString, JsValue, NativeFunction, Source, string::utf16};
use boa_engine::object::builtins::{JsArray, JsMap};
use boa_engine::object::FunctionObjectBuilder;
use boa_engine::object::ObjectKind::Array;
use boa_engine::property::{Attribute, PropertyDescriptor};
use boa_gc::{Finalize, GcRefCell, Trace};
use boa_runtime::Console;
use chrono::Local;
use log::LevelFilter;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use handler::Handler;
use crate::app_config::AppConfig;

use crate::grab::grab_ex;
use crate::js::JS;
use crate::shortcut::parse_shortcut;

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

    let shortcut = parse_shortcut(app_config.repeat_shortcut.as_str())?;

    thread::spawn(move || {
        log::debug!("Starting handler thread: {:?}", thread::current().id());

        let mut js = JS::new().expect("Cannot create JS instance");
        let src = include_str!("../js/dynamic-macro.js");
        js.eval(src.to_string()).unwrap();

        let mut handler = Handler::new(64, shortcut, js);
        if let Err(error) = grab_ex(move |event, cg_event_type, cg_event_ref| {
            handler.callback(event, cg_event_type, cg_event_ref)
        }) {
            println!("Error: {:?}", error)
        }
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
