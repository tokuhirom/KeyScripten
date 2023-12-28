use apple_sys::CoreGraphics::{CGEventFlags, CGKeyCode};

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
