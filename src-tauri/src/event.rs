use apple_sys::CoreGraphics::{CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp};

pub fn event_type(cg_event_type: CGEventType) -> &'static str {
    #[allow(non_upper_case_globals)]
    match cg_event_type {
        CGEventType_kCGEventKeyDown => "keydown",
        CGEventType_kCGEventKeyUp => "keyup",
        CGEventType_kCGEventFlagsChanged => "flags_changed",
        _ => "unknown",
    }
}
