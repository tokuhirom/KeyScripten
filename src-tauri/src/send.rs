use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventCreate, CGEventCreateKeyboardEvent, CGEventField_kCGEventSourceUserData, CGEventFlags, CGEventPost, CGEventSetFlags, CGEventSetIntegerValueField, CGEventSetType, CGEventSourceCreate, CGEventSourceRef, CGEventSourceStateID_kCGEventSourceStatePrivate, CGEventTapLocation_kCGHIDEventTap, CGEventType_kCGEventFlagsChanged, CGKeyCode};

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

pub const USER_DATA_FOR_ONE_MORE_TIME: i64 = 5963;

fn build_event_source() -> anyhow::Result<CGEventSourceRef> {
    unsafe {
        let source = CGEventSourceCreate(CGEventSourceStateID_kCGEventSourceStatePrivate);
        if source.is_null() {
            return Err(anyhow!("Cannot create event source"));
        }
        Ok(source)
    }
}

pub fn send_keyboard_event(keycode: CGKeyCode, flags: CGEventFlags, keydown: bool) -> anyhow::Result<()> {
    let source = build_event_source()?;

    log::debug!("Sending keyboard event: {:?}", keycode);
    unsafe {
        let event = CGEventCreateKeyboardEvent(source, keycode, keydown);
        if event.is_null() {
            return Err(anyhow!("Cannot create keyboard event"));
        }
        CGEventSetFlags(event, flags);
        CGEventSetIntegerValueField(event, CGEventField_kCGEventSourceUserData, USER_DATA_FOR_ONE_MORE_TIME);
        CGEventPost(CGEventTapLocation_kCGHIDEventTap, event);
        Ok(())
    }
}

pub fn send_flags_changed_event(flags: CGEventFlags) -> anyhow::Result<()> {
    let source = build_event_source()?;

    unsafe {
        let event = CGEventCreate(source);
        if event.is_null() {
            return Err(anyhow!("Can't create new CGEvent"));
        }
        CGEventSetType(event, CGEventType_kCGEventFlagsChanged);
        CGEventSetFlags(event, flags);
        CGEventPost(CGEventTapLocation_kCGHIDEventTap, event);
        Ok(())
    }
}
