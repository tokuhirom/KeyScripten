use anyhow::anyhow;
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGEventType, CGKeyCode};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

fn build_event_source() -> anyhow::Result<CGEventSource> {
    CGEventSource::new(CGEventSourceStateID::Private)
        .map_err(|err| { anyhow!("Cannot create event source: {:?}", err) })
}

pub fn send_keyboard_event(keycode: CGKeyCode, keydown: bool) -> anyhow::Result<()> {
    let source = build_event_source()?;

    log::info!("Sending keyboard event: {:?}", keycode);
    let event = CGEvent::new_keyboard_event(source, keycode, keydown)
        .map_err(|err| { anyhow!("Cannot create keyboard event")})?;
    event.post(CGEventTapLocation::HID);
    Ok(())
}

pub fn send_flags_changed_event(flags: CGEventFlags) -> anyhow::Result<()> {
    let source = build_event_source()?;

    let event = CGEvent::new(source)
        .map_err(|err| { anyhow!("Can't create new CGEvent: {:?}", err)})?;
    event.set_type(CGEventType::FlagsChanged);
    event.set_flags(flags);
    event.post(CGEventTapLocation::HID);
    Ok(())
}
