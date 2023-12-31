use apple_sys::CoreGraphics::CGEventFlags;
use apple_sys::CoreGraphics::{CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp, CGKeyCode};

/// In order to manage different OSs, the current EventType choices are a mix and
/// match to account for all possible events.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Event {
    /// The keys correspond to a standard qwerty layout, they don't correspond
    /// To the actual letter a user would use, that requires some layout logic to be added.
    KeyPress(CGKeyCode),
    KeyRelease(CGKeyCode),
    FlagsChanged(CGKeyCode, CGEventFlags),
}

pub fn event_type(cg_event_type: CGEventType) -> &'static str {
    #[allow(non_upper_case_globals)]
    match cg_event_type {
        CGEventType_kCGEventKeyDown => "keydown",
        CGEventType_kCGEventKeyUp => "keyup",
        CGEventType_kCGEventFlagsChanged => "flags_changed",
        _ => "unknown",
    }
}
