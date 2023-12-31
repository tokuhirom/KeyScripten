#![allow(improper_ctypes_definitions)]

use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CFMachPortCreateRunLoopSource, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun, kCFAllocatorDefault, kCFRunLoopCommonModes};
use apple_sys::CoreGraphics::{CGEventField_kCGEventSourceUserData, CGEventGetIntegerValueField, CGEventMask, CGEventRef, CGEventSetType, CGEventTapCreate, CGEventTapEnable, CGEventTapLocation_kCGHIDEventTap, CGEventTapOptions_kCGEventTapOptionDefault, CGEventTapPlacement_kCGHeadInsertEventTap, CGEventTapProxy, CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp, CGEventType_kCGEventNull};
use crate::js::JS;
use crate::send::USER_DATA_FOR_ONE_MORE_TIME;

// TODO don't use global variable here.
static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(CGEventType, CGEventRef) -> bool>> = None;
static mut GLOBAL_JS: Option<Box<JS<'_>>> = None;

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

// This event is sent from this application itself.
unsafe fn is_sent_from_this_app(cg_event: CGEventRef) -> bool {
    let user_data = CGEventGetIntegerValueField(cg_event, CGEventField_kCGEventSourceUserData);
    user_data == USER_DATA_FOR_ONE_MORE_TIME
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

    let Some(js) = &mut GLOBAL_JS else {
        return cg_event;
    };

    match js.send_event(event_type, cg_event) {
        Ok(b) => {
            if !b {
                CGEventSetType(cg_event, CGEventType_kCGEventNull);
            }
        }
        Err(err) => {
            log::error!("Cannot call JS callback: {:?}", err);
        }
    }

    cg_event
}

pub fn grab_ex() -> anyhow::Result<()> {
    unsafe {
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

pub fn run_handler() {
    let mut js = JS::new().expect("Cannot create JS instance");
    let src = include_str!("../js/dynamic-macro.js");
    js.eval(src.to_string()).unwrap();
    unsafe {
        GLOBAL_JS = Some(Box::new(js));
    }

    if let Err(error) = grab_ex() {
        println!("Error: {:?}", error)
    }
}
