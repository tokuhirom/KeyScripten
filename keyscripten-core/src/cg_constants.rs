use apple_sys::CoreGraphics::{
    CGEventField, CGEventFlags, CGEventSourceStateID, CGEventTapLocation, CGEventTapOptions,
    CGEventTapPlacement, CGEventType,
};

// CGEventType
pub const kCGEventNull: CGEventType = 0;
pub const kCGEventKeyDown: CGEventType = 10;
pub const kCGEventKeyUp: CGEventType = 11;
pub const kCGEventFlagsChanged: CGEventType = 12;

// CGEventField
pub const kCGKeyboardEventKeycode: CGEventField = 9;
pub const kCGEventSourceUserData: CGEventField = 100;

// CGEventTapLocation
pub const kCGHIDEventTap: CGEventTapLocation = 0;

// CGEventTapPlacement
pub const kCGHeadInsertEventTap: CGEventTapPlacement = 0;

// CGEventTapOptions
pub const kCGEventTapOptionDefault: CGEventTapOptions = 0;

// CGEventSourceStateID (-1 = kCGEventSourceStatePrivate)
pub const kCGEventSourceStatePrivate: CGEventSourceStateID = -1;

// CGEventFlags (modifier keys)
pub const kCGEventFlagMaskAlphaShift: CGEventFlags = 0x00010000;
pub const kCGEventFlagMaskShift: CGEventFlags = 0x00020000;
pub const kCGEventFlagMaskControl: CGEventFlags = 0x00040000;
pub const kCGEventFlagMaskAlternate: CGEventFlags = 0x00080000;
pub const kCGEventFlagMaskCommand: CGEventFlags = 0x00100000;
pub const kCGEventFlagMaskHelp: CGEventFlags = 0x00400000;
pub const kCGEventFlagMaskSecondaryFn: CGEventFlags = 0x00800000;
pub const kCGEventFlagMaskNumericPad: CGEventFlags = 0x00200000;
pub const kCGEventFlagMaskNonCoalesced: CGEventFlags = 0x01000000;
