mod send;
mod grab;
mod event;
mod sender;
mod state;
mod handler;
mod app_config;
mod keycode;
mod shortcut;

use apple_sys::CoreGraphics::{CGEventFlags, CGKeyCode};
use simplelog::ColorChoice;
use handler::Handler;
use crate::app_config::AppConfig;

use crate::grab::grab_ex;
use crate::shortcut::parse_shortcut;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> anyhow::Result<()> {
    let log_config = simplelog::ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("Cannot get timezone")
        .build();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            log_config.clone(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto
        ),
    ])?;

    log::info!("Loading configuration from {:?}", AppConfig::get_configuration_file_path()?);
    let app_config = AppConfig::load()?;
    log::info!("Shortcut key is: `{}`", app_config.repeat_shortcut);

    let shortcut = parse_shortcut(app_config.repeat_shortcut.as_str())?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let mut handler = Handler::new(64, shortcut);
    if let Err(error) = grab_ex(move |event| {
        handler.callback(event)
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct KeyState {
    code: CGKeyCode,
    flags: CGEventFlags,
}
