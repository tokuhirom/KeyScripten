use std::collections::HashMap;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventFlags, CGEventFlags_kCGEventFlagMaskAlternate, CGEventFlags_kCGEventFlagMaskCommand, CGEventFlags_kCGEventFlagMaskControl, CGEventFlags_kCGEventFlagMaskShift, CGKeyCode};
use crate::keycode;

#[derive(Debug, PartialEq)]
pub struct HotKey {
    pub flags: CGEventFlags,
    pub keycode: CGKeyCode,
}

const KEY_CODE_KEY_T: CGKeyCode = 17;

impl HotKey {
    pub fn matches(&self, flags: CGEventFlags, code: CGKeyCode) -> bool {
        let expected_flags = self.flags;
        let expected_code = self.keycode;

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
}

impl Default for HotKey {
    fn default() -> Self {
        HotKey {
            flags: CGEventFlags_kCGEventFlagMaskControl,
            keycode: KEY_CODE_KEY_T,
        }
    }
}

pub fn parse_hotkey(s: &str) -> anyhow::Result<HotKey> {
    let mut map = HashMap::new();
    map.insert("C-", CGEventFlags_kCGEventFlagMaskControl);
    map.insert("S-", CGEventFlags_kCGEventFlagMaskShift);
    map.insert("M-", CGEventFlags_kCGEventFlagMaskCommand);
    map.insert("A-", CGEventFlags_kCGEventFlagMaskAlternate);

    let mut start = 0;
    let mut flags = 0;

    while s.len() - start >= 2 {
        let part = &s[start..start+2];
        if let Some(code) = map.get(part) {
            flags |= *code;
            start += 2;
        }
        else {
            break;
        }
    }

    if start >= s.len() {
        Err(anyhow!("Cannot parse shortcut: `{:?}`", s))
    }
    else {
        let keyname = &s[start..];
        match keycode::get_keycode(keyname) {
            Some(keycode) => Ok(HotKey {
                flags,
                keycode
            }),
            None => {
                Err(anyhow!("Unknown key: `{:?}`", s))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shortcut() -> anyhow::Result<()> {
        // 指定したフラグとキーコードが正しくパースされることをテスト
        let shortcut = parse_hotkey("C-M-t")?;
        assert_eq!(shortcut.flags, CGEventFlags_kCGEventFlagMaskControl | CGEventFlags_kCGEventFlagMaskCommand);
        assert_eq!(shortcut.keycode, KEY_CODE_KEY_T);

        // 未知のキーコードが与えられた場合にエラーになること
        assert!(parse_hotkey("C-unknown").is_err());

        // 無効なショートカット文字列がエラーを返すことをテスト
        assert!(parse_hotkey("").is_err());
        assert!(parse_hotkey("C-").is_err());
        assert!(parse_hotkey("unknown").is_err());

        Ok(())
    }
}