use apple_sys::CoreGraphics::{CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp};

pub fn event_type(cg_event_type: CGEventType) -> &'static str {
    #[allow(non_upper_case_globals)]
    match cg_event_type {
        CGEventType_kCGEventKeyDown => "keyDown",
        CGEventType_kCGEventKeyUp => "keyUp",
        CGEventType_kCGEventFlagsChanged => "flagsChanged",
        _ => "unknown",
    }
}
