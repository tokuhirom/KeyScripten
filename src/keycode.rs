use core_graphics::event::CGKeyCode;
use std::collections::HashMap;

type KeyCodeMap = HashMap<&'static str, CGKeyCode>;

fn build_key_code_map() -> KeyCodeMap {
    let mut map: KeyCodeMap = HashMap::new();
    map.insert("alt", 58);
    map.insert("alt_gr", 61);
    map.insert("backspace", 51);
    map.insert("caps_lock", 57);
    map.insert("control_left", 59);
    map.insert("control_right", 62);
    map.insert("down_arrow", 125);
    map.insert("escape", 53);
    map.insert("f1", 122);
    map.insert("f10", 109);
    map.insert("f11", 103);
    map.insert("f12", 111);
    map.insert("f2", 120);
    map.insert("f3", 99);
    map.insert("f4", 118);
    map.insert("f5", 96);
    map.insert("f6", 97);
    map.insert("f7", 98);
    map.insert("f8", 100);
    map.insert("f9", 101);
    map.insert("function", 63);
    map.insert("left_arrow", 123);
    map.insert("meta_left", 55);
    map.insert("meta_right", 54);
    map.insert("return", 36);
    map.insert("enter", 36);
    map.insert("right_arrow", 124);
    map.insert("shift_left", 56);
    map.insert("shift_right", 60);
    map.insert("space", 49); // space
    map.insert("tab", 48); // tab
    map.insert("up", 126); // up arrow
    map.insert("`", 50); // backquote
    map.insert("num1", 18);
    map.insert("num2", 19);
    map.insert("num3", 20);
    map.insert("num4", 21);
    map.insert("num5", 23);
    map.insert("num6", 22);
    map.insert("num7", 26);
    map.insert("num8", 28);
    map.insert("num9", 25);
    map.insert("num0", 29);
    map.insert("-", 27); // minus
    map.insert("=", 24); // equal
    map.insert("q", 12);
    map.insert("w", 13);
    map.insert("e", 14);
    map.insert("r", 15);
    map.insert("t", 17);
    map.insert("y", 16);
    map.insert("u", 32);
    map.insert("i", 34);
    map.insert("o", 31);
    map.insert("p", 35);
    map.insert("{", 33); // left_bracket
    map.insert("}", 30); // right_bracket
    map.insert("a", 0);
    map.insert("s", 1);
    map.insert("d", 2);
    map.insert("f", 3);
    map.insert("g", 5);
    map.insert("h", 4);
    map.insert("j", 38);
    map.insert("k", 40);
    map.insert("l", 37);
    map.insert(";", 41); // semicolon
    map.insert("'", 39); // quote
    map.insert("\\", 42); // backslash
    map.insert("z", 6);
    map.insert("x", 7);
    map.insert("c", 8);
    map.insert("v", 9);
    map.insert("b", 11);
    map.insert("n", 45);
    map.insert("m", 46);
    map.insert(",", 43); // comma
    map.insert(".", 47); // dot
    map.insert("/", 44); // slash
    map
}

pub(crate) fn get_keycode(keyname: &str) -> Option<CGKeyCode> {
    let map = build_key_code_map();
    map.get(&*keyname.to_ascii_lowercase()).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_keycode() {
        assert_eq!(get_keycode("caps_lock"), Some(57));
        assert_eq!(get_keycode("shift_left"), Some(56));
        assert_eq!(get_keycode("a"), Some(0));
        assert_eq!(get_keycode("A"), Some(0));
        assert_eq!(get_keycode("non_existing_key"), None);
    }
}
