use std::collections::VecDeque;
use apple_sys::CoreGraphics::{CGEventFlags, CGEventFlags_kCGEventFlagMaskAlternate, CGEventFlags_kCGEventFlagMaskCommand, CGEventFlags_kCGEventFlagMaskControl, CGEventFlags_kCGEventFlagMaskNonCoalesced, CGEventFlags_kCGEventFlagMaskShift, CGEventRef, CGEventType, CGKeyCode};
use crate::event::{Event};
use crate::js::JS;
use crate::KeyState;

use crate::sender::Sender;
use crate::shortcut::Shortcut;
use crate::state::State;

pub struct Handler<'a> {
    buffer: VecDeque<KeyState>,
    capacity: usize,
    latest_flags: CGEventFlags,
    shortcut: Shortcut,
    js: JS<'a>,
}

impl Handler<'_> {
    pub fn new(capacity: usize, shortcut: Shortcut, js: JS) -> Handler {
        Handler {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            latest_flags: CGEventFlags_kCGEventFlagMaskNonCoalesced,
            shortcut,
            js,
        }
    }

    pub fn callback(&mut self, event: Event, cg_event_type: CGEventType, cg_event_ref: CGEventRef) -> Option<Event> {
        return match self.js.send_event(cg_event_type, cg_event_ref) {
            Ok(b) => {
                if b {
                    Some(event)
                } else {
                    None
                }
            }
            Err(err) => {
                log::error!("Cannot call JS callback: {:?}", err);
                None
            }
        };
    }
}

pub fn matches_hotkey_string(flags: CGEventFlags, code: CGKeyCode, shortcut: &Shortcut) -> bool {
    let expected_flags = shortcut.flags;
    let expected_code = shortcut.keycode;

    log::debug!("is_shortcut_pressed?: flags={:?} code={:?}, expected({:?}, {:?})",
        flags, code,
        expected_flags, expected_code);

    // 全てのキー修飾フラグを取得
    let all_modifiers = CGEventFlags_kCGEventFlagMaskControl
        | CGEventFlags_kCGEventFlagMaskAlternate
        | CGEventFlags_kCGEventFlagMaskShift
        | CGEventFlags_kCGEventFlagMaskCommand;

    // 期待するフラグだけが押されていて、それ以外のフラグは押されていないことをチェック
    let is_correct_flags_pressed = flags & all_modifiers == expected_flags;

    // キーコードが期待通りであることをチェック
    let is_correct_keycode = code == expected_code;

    is_correct_flags_pressed && is_correct_keycode
}

