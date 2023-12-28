#![allow(improper_ctypes_definitions)]
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use core_graphics::event::{CGEventTapLocation, CGEventType};
use std::os::raw::c_void;
use anyhow::anyhow;
use crate::common::{CFMachPortCreateRunLoopSource, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRun, CGEventRef, CGEventTapCreate, CGEventTapEnable, CGEventTapOption, CGEventTapProxy, convert, kCFRunLoopCommonModes, kCGHeadInsertEventTap};
use crate::event::Event;

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event) -> Option<Event>>> = None;

#[link(name = "Cocoa", kind = "framework")]
extern "C" {}

unsafe extern "C" fn raw_callback(
    _proxy: CGEventTapProxy,
    _type: CGEventType,
    cg_event: CGEventRef,
    _user_info: *mut c_void,
) -> CGEventRef {
    // println!("Event ref {:?}", cg_event_ptr);
    // let cg_event: CGEvent = transmute_copy::<*mut c_void, CGEvent>(&cg_event_ptr);
    if let Some(event) = convert(_type, &cg_event) {
        if let Some(callback) = &mut GLOBAL_CALLBACK {
            if callback(event).is_none() {
                cg_event.set_type(CGEventType::Null);
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
            CGEventTapLocation::HID, // HID, Session, AnnotatedSession,
            kCGHeadInsertEventTap,
            CGEventTapOption::Default,
            (1 << CGEventType::KeyDown as u64)
                + (1 << CGEventType::KeyUp as u64)
                + (1 << CGEventType::FlagsChanged as u64),
            raw_callback,
            nil,
        );
        if tap.is_null() {
            return Err(anyhow!("Cannot create CGEventTapCreate"));
        }
        let _loop = CFMachPortCreateRunLoopSource(nil, tap, 0);
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
