use apple_sys::CoreGraphics::{CGEventFlags, CGEventFlags_kCGEventFlagMaskAlternate, CGEventFlags_kCGEventFlagMaskCommand, CGEventFlags_kCGEventFlagMaskControl, CGEventFlags_kCGEventFlagMaskNonCoalesced, CGEventFlags_kCGEventFlagMaskShift, CGEventRef, CGEventType, CGKeyCode};
use crate::shortcut::Shortcut;

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

