use apple_sys::CoreGraphics::{
    CGEventField_kCGKeyboardEventKeycode, CGEventGetFlags, CGEventGetIntegerValueField, CGEventRef,
    CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown,
    CGEventType_kCGEventKeyUp,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn event_type(cg_event_type: CGEventType) -> &'static str {
    #[allow(non_upper_case_globals)]
    match cg_event_type {
        CGEventType_kCGEventKeyDown => "keyDown",
        CGEventType_kCGEventKeyUp => "keyUp",
        CGEventType_kCGEventFlagsChanged => "flagsChanged",
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

impl Event {
    pub fn from_cf(cg_event_type: CGEventType, cg_event_ref: CGEventRef) -> Self {
        unsafe {
            let keycode =
                CGEventGetIntegerValueField(cg_event_ref, CGEventField_kCGKeyboardEventKeycode);
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
