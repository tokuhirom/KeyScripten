use anyhow::anyhow;
use core_graphics::event::{
    CGEvent, CGEventTapLocation,
};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use crate::event::Event;

unsafe fn convert_native_with_source(
    event_type: &Event,
    source: CGEventSource,
) -> Option<CGEvent> {
    match event_type {
        Event::KeyPress(code) => {
            log::info!("[rdev] Sending key press event: {:?}", code);
            CGEvent::new_keyboard_event(source, *code, true).ok()
        }
        Event::KeyRelease(code) => {
            log::info!("[rdev] Sending key release event: {:?}", code);
            CGEvent::new_keyboard_event(source, *code, false).ok()
        }
        _ => {
            return None
        }
    }
}

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

pub fn simulate_ex(event_type: &Event) -> anyhow::Result<()> {
    unsafe {
        let source = match CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
            Ok(source) => {
                source
            }
            Err(err) => {
                return Err(anyhow!("Cannot create event source: {:?}", err));
            }
        };
        if let Some(cg_event) = convert_native_with_source(event_type, source) {
            cg_event.post(CGEventTapLocation::HID);
            Ok(())
        } else {
            Err(anyhow!("Cannot create native event"))
        }
    }
}
