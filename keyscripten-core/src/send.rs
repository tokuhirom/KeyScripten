use anyhow::anyhow;
use apple_sys::CoreGraphics::{
    CGEventCreate, CGEventCreateKeyboardEvent, CGEventFlags, CGEventPost, CGEventSetFlags,
    CGEventSetIntegerValueField, CGEventSetType, CGEventSourceCreate, CGEventSourceRef, CGKeyCode,
};
use crate::cg_constants::{
    kCGEventFlagsChanged, kCGEventSourceStatePrivate, kCGEventSourceUserData, kCGHIDEventTap,
};

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

pub const USER_DATA_FROM_THIS_APP: i64 = 5963;

fn build_event_source() -> anyhow::Result<CGEventSourceRef> {
    unsafe {
        let source = CGEventSourceCreate(kCGEventSourceStatePrivate);
        if source.is_null() {
            return Err(anyhow!("Cannot create event source"));
        }
        Ok(source)
    }
}

pub fn send_keyboard_event(
    keycode: CGKeyCode,
    flags: CGEventFlags,
    keydown: bool,
) -> anyhow::Result<()> {
    let source = build_event_source()?;

    log::debug!("Sending keyboard event: {:?}", keycode);
    unsafe {
        let event = CGEventCreateKeyboardEvent(source, keycode, keydown);
        if event.is_null() {
            return Err(anyhow!("Cannot create keyboard event"));
        }
        CGEventSetFlags(event, flags);
        CGEventSetIntegerValueField(
            event,
            kCGEventSourceUserData,
            USER_DATA_FROM_THIS_APP,
        );
        CGEventPost(kCGHIDEventTap, event);
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
        CGEventSetType(event, kCGEventFlagsChanged);
        CGEventSetFlags(event, flags);
        CGEventPost(kCGHIDEventTap, event);
        Ok(())
    }
}
