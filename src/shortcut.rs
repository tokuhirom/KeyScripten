use std::collections::HashMap;
use core_graphics::event::{CGEventFlags, CGKeyCode};
use anyhow::anyhow;
use crate::{keycode, keycodes};
use crate::key::Key;

#[derive(Debug, PartialEq)]
pub struct Shortcut {
    pub flags: CGEventFlags,
    pub keycode: CGKeyCode,
}

impl Default for Shortcut {
    fn default() -> Self {
        Shortcut {
            flags: CGEventFlags::CGEventFlagControl,
            keycode: keycodes::code_from_key(Key::KeyT).unwrap(),
        }
    }
}

pub fn parse_shortcut(s: &str) -> anyhow::Result<Shortcut> {
    let mut map = HashMap::new();
    map.insert("C-", CGEventFlags::CGEventFlagControl);
    map.insert("S-", CGEventFlags::CGEventFlagShift);
    map.insert("M-", CGEventFlags::CGEventFlagCommand);
    map.insert("A-", CGEventFlags::CGEventFlagAlternate);

    let mut start = 0;
    let mut flags = CGEventFlags::CGEventFlagNull;

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
    use core_graphics::event::CGEventFlags;
    use crate::keycodes;

    #[test]
    fn test_parse_shortcut() -> anyhow::Result<()> {
        // 指定したフラグとキーコードが正しくパースされることをテスト
        let shortcut = parse_shortcut("C-M-t")?;
        assert_eq!(shortcut.flags, CGEventFlags::CGEventFlagControl | CGEventFlags::CGEventFlagCommand);
        assert_eq!(shortcut.keycode, keycodes::code_from_key(Key::KeyT).unwrap());

        // 未知のキーコードが与えられた場合にエラーになること
        assert!(parse_shortcut("C-unknown").is_err());

        // 無効なショートカット文字列がエラーを返すことをテスト
        assert!(parse_shortcut("").is_err());
        assert!(parse_shortcut("C-").is_err());
        assert!(parse_shortcut("unknown").is_err());

        Ok(())
    }
}