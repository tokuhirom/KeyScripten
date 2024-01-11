use apple_sys::CoreGraphics::{
    CGEventField_kCGKeyboardEventKeycode, CGEventGetFlags, CGEventGetIntegerValueField, CGEventRef,
    CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown,
    CGEventType_kCGEventKeyUp,
};
use serde::{Deserialize, Serialize};

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
}

impl Event {
    pub fn from_cf(cg_event_type: CGEventType, cg_event_ref: CGEventRef) -> Self {
        unsafe {
            let keycode =
                CGEventGetIntegerValueField(cg_event_ref, CGEventField_kCGKeyboardEventKeycode);
            let flags = CGEventGetFlags(cg_event_ref);

            Event {
                event_type: event_type(cg_event_type).to_string(),
                keycode,
                flags,
            }
        }
    }
}
