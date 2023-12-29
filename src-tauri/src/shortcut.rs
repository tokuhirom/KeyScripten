use std::collections::HashMap;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventFlags, CGEventFlags_kCGEventFlagMaskAlternate, CGEventFlags_kCGEventFlagMaskCommand, CGEventFlags_kCGEventFlagMaskControl, CGEventFlags_kCGEventFlagMaskShift, CGKeyCode};
use crate::keycode;

#[derive(Debug, PartialEq)]
pub struct Shortcut {
    pub flags: CGEventFlags,
    pub keycode: CGKeyCode,
}

const KEY_CODE_KEY_T: CGKeyCode = 17;

impl Default for Shortcut {
    fn default() -> Self {
        Shortcut {
            flags: CGEventFlags_kCGEventFlagMaskControl,
            keycode: KEY_CODE_KEY_T,
        }
    }
}

pub fn parse_shortcut(s: &str) -> anyhow::Result<Shortcut> {
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
            Some(keycode) => Ok(Shortcut {
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
        let shortcut = parse_shortcut("C-M-t")?;
        assert_eq!(shortcut.flags, CGEventFlags_kCGEventFlagMaskControl | CGEventFlags_kCGEventFlagMaskCommand);
        assert_eq!(shortcut.keycode, KEY_CODE_KEY_T);

        // 未知のキーコードが与えられた場合にエラーになること
        assert!(parse_shortcut("C-unknown").is_err());

        // 無効なショートカット文字列がエラーを返すことをテスト
        assert!(parse_shortcut("").is_err());
        assert!(parse_shortcut("C-").is_err());
        assert!(parse_shortcut("unknown").is_err());

        Ok(())
    }
}