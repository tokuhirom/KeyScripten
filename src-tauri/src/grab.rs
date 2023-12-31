#![allow(improper_ctypes_definitions)]

use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CFMachPortCreateRunLoopSource, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun, kCFAllocatorDefault, kCFRunLoopCommonModes};
use apple_sys::CoreGraphics::{CGEventField_kCGEventSourceUserData, CGEventField_kCGKeyboardEventKeycode, CGEventGetFlags, CGEventGetIntegerValueField, CGEventMask, CGEventRef, CGEventSetType, CGEventTapCreate, CGEventTapEnable, CGEventTapLocation_kCGHIDEventTap, CGEventTapOptions_kCGEventTapOptionDefault, CGEventTapPlacement_kCGHeadInsertEventTap, CGEventTapProxy, CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp, CGEventType_kCGEventNull, CGKeyCode};
use crate::event::Event;
use crate::send::USER_DATA_FOR_ONE_MORE_TIME;

// TODO don't use global variable here.
static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(CGEventType, CGEventRef) -> bool>> = None;

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

// This event is sent from this application itself.
unsafe fn is_sent_from_this_app(cg_event: CGEventRef) -> bool {
    let user_data = CGEventGetIntegerValueField(cg_event, CGEventField_kCGEventSourceUserData);
    return user_data == USER_DATA_FOR_ONE_MORE_TIME;
}

unsafe fn convert(
    cg_event_type: CGEventType,
    cg_event: CGEventRef,
) -> Option<Event> {
    let user_data = CGEventGetIntegerValueField(cg_event, CGEventField_kCGEventSourceUserData);
    log::debug!("event's USER_DATA={}", user_data);
    if user_data == USER_DATA_FOR_ONE_MORE_TIME {
        // This event is sent from this application itself.
        return None
    }

    #[allow(non_upper_case_globals)]
    match cg_event_type {
        CGEventType_kCGEventKeyDown => {
            let code = CGEventGetIntegerValueField(cg_event, CGEventField_kCGKeyboardEventKeycode);
            Some(Event::KeyPress(code as CGKeyCode))
        }
        CGEventType_kCGEventKeyUp => {
            let code = CGEventGetIntegerValueField(cg_event, CGEventField_kCGKeyboardEventKeycode);
            Some(Event::KeyRelease(code as CGKeyCode))
        }
        CGEventType_kCGEventFlagsChanged => {
            let code = CGEventGetIntegerValueField(cg_event, CGEventField_kCGKeyboardEventKeycode);
            let flags = CGEventGetFlags(cg_event);
            Some(Event::FlagsChanged(code as CGKeyCode, flags))
        }
        _ => {
            None
        }
    }
}


unsafe extern "C" fn raw_callback(
    _proxy: CGEventTapProxy,
    event_type: CGEventType,
    cg_event: CGEventRef,
    _user_info: *mut ::std::os::raw::c_void,
) -> CGEventRef {
    log::debug!("Called raw_callback");

    if is_sent_from_this_app(cg_event) {
        return cg_event;
    }

    let Some(callback) = &mut GLOBAL_CALLBACK else {
        return cg_event;
    };
    if !callback(event_type, cg_event) {
        CGEventSetType(cg_event, CGEventType_kCGEventNull);
    }
    cg_event
}

pub fn grab_ex<T>(callback: T) -> anyhow::Result<()>
where
    T: FnMut(CGEventType, CGEventRef) -> bool + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        let _pool = NSAutoreleasePool::new(nil);
        log::debug!("Calling CGEventTapCreate");
        let tap = CGEventTapCreate(
            CGEventTapLocation_kCGHIDEventTap, // HID, Session, AnnotatedSession,
            CGEventTapPlacement_kCGHeadInsertEventTap,
            CGEventTapOptions_kCGEventTapOptionDefault,
            (1 << CGEventType_kCGEventKeyDown as CGEventMask)
                + (1 << CGEventType_kCGEventKeyUp as CGEventMask)
                + (1 << CGEventType_kCGEventFlagsChanged as CGEventMask),
            Some(raw_callback),
            std::ptr::null_mut(), // TODO use callback here!!! Do not use global variable.
        );
        if tap.is_null() {
            return Err(anyhow!("Cannot create CGEventTapCreate"));
        }
        let _loop = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, tap, 0);
        if _loop.is_null() {
            return Err(anyhow!("Error in CFMachPortCreateRunLoopSource"));
        }

        let current_loop = CFRunLoopGetCurrent();
        log::debug!("Calling CFRunLoopAddSource");
        CFRunLoopAddSource(current_loop, _loop, kCFRunLoopCommonModes);

        CGEventTapEnable(tap, true);
        log::info!("Running CFRunLoopRun");
        CFRunLoopRun();
    }
    Ok(())
}
