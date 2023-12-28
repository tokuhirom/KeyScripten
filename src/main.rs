mod send;
mod keycodes;
mod grab;
mod common;
mod event;
mod key;
mod sender;
mod state;
mod handler;

use core_graphics::event::{CGEventFlags, CGKeyCode};
use simplelog::ColorChoice;
use handler::Handler;

use crate::grab::grab_ex;

fn main() -> anyhow::Result<()> {
    let config = simplelog::ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("Cannot get timezone")
        .build();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto
        ),
    ])?;

    let mut handler = Handler::new(64);
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
