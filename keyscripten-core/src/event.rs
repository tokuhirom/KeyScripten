use apple_sys::CoreGraphics::{CGEventGetFlags, CGEventGetIntegerValueField, CGEventRef, CGEventType};
use crate::cg_constants::{
    kCGEventFlagsChanged, kCGEventKeyDown, kCGEventKeyUp, kCGKeyboardEventKeycode,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn event_type(cg_event_type: CGEventType) -> &'static str {
    #[allow(non_upper_case_globals)]
    match cg_event_type {
        kCGEventKeyDown => "keyDown",
        kCGEventKeyUp => "keyUp",
        kCGEventFlagsChanged => "flagsChanged",
        _ => "unknown",
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub event_type: String,
    pub keycode: i64,
    pub flags: u64,
    pub timestamp: u64,
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl Event {
    pub fn from_cf(cg_event_type: CGEventType, cg_event_ref: CGEventRef) -> Self {
        unsafe {
            let keycode =
                CGEventGetIntegerValueField(cg_event_ref, kCGKeyboardEventKeycode);
            let flags = CGEventGetFlags(cg_event_ref);

            let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => 0,
            };

            Event {
                timestamp,
                event_type: event_type(cg_event_type).to_string(),
                keycode,
                flags,
            }
        }
    }
}
