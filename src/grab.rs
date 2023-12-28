#![allow(improper_ctypes_definitions)]

use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CFMachPortCreateRunLoopSource, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun, kCFAllocatorDefault, kCFRunLoopCommonModes};
use apple_sys::CoreGraphics::{CGEventField_kCGEventSourceUserData, CGEventField_kCGKeyboardEventKeycode, CGEventGetFlags, CGEventGetIntegerValueField, CGEventMask, CGEventRef, CGEventSetType, CGEventTapCreate, CGEventTapEnable, CGEventTapLocation_kCGHIDEventTap, CGEventTapOptions_kCGEventTapOptionDefault, CGEventTapPlacement_kCGHeadInsertEventTap, CGEventTapProxy, CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp, CGEventType_kCGEventNull, CGKeyCode};
use crate::event::Event;
use crate::send::USER_DATA_FOR_ONE_MORE_TIME;

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event) -> Option<Event>>> = None;

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

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
    println!("Event ref {:?}, {:?}", event_type, cg_event,);
    // let cg_event: CGEvent = transmute_copy::<*mut c_void, CGEvent>(&cg_event_ptr);
    if let Some(event) = convert(event_type, cg_event) {
        if let Some(callback) = &mut GLOBAL_CALLBACK {
            if callback(event).is_none() {
                CGEventSetType(cg_event, CGEventType_kCGEventNull);
            }
        }
    }
    cg_event
}

pub fn grab_ex<T>(callback: T) -> anyhow::Result<()>
where
    T: FnMut(Event) -> Option<Event> + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        let _pool = NSAutoreleasePool::new(nil);
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
        CFRunLoopAddSource(current_loop, _loop, kCFRunLoopCommonModes);

        CGEventTapEnable(tap, true);
        CFRunLoopRun();
    }
    Ok(())
}
