use boa_engine::class::{Class, ClassBuilder};
use boa_engine::property::Attribute;
use boa_engine::{js_string, Context, JsObject, JsResult, JsValue};
use boa_gc::{Finalize, Trace};

#[derive(Debug, Trace, Finalize)]
pub struct JsKeyCode {}

pub fn build_keycode(context: &mut Context) -> JsResult<JsObject> {
    let object = JsObject::with_null_proto();
    object.set(js_string!("ALT"), JsValue::Integer(58), false, context)?;
    object.set(js_string!("ALT_GR"), JsValue::Integer(61), false, context)?;
    object.set(
        js_string!("BACKSPACE"),
        JsValue::Integer(51),
        false,
        context,
    )?;
    object.set(
        js_string!("CAPS_LOCK"),
        JsValue::Integer(57),
        false,
        context,
    )?;
    object.set(
        js_string!("CONTROL_LEFT"),
        JsValue::Integer(59),
        false,
        context,
    )?;
    object.set(
        js_string!("CONTROL_RIGHT"),
        JsValue::Integer(62),
        false,
        context,
    )?;
    object.set(
        js_string!("DOWN_ARROW"),
        JsValue::Integer(125),
        false,
        context,
    )?;
    object.set(js_string!("ESCAPE"), JsValue::Integer(53), false, context)?;
    object.set(js_string!("F1"), JsValue::Integer(122), false, context)?;
    object.set(js_string!("F10"), JsValue::Integer(109), false, context)?;
    object.set(js_string!("F11"), JsValue::Integer(103), false, context)?;
    object.set(js_string!("F12"), JsValue::Integer(111), false, context)?;
    object.set(js_string!("F2"), JsValue::Integer(120), false, context)?;
    object.set(js_string!("F3"), JsValue::Integer(99), false, context)?;
    object.set(js_string!("F4"), JsValue::Integer(118), false, context)?;
    object.set(js_string!("F5"), JsValue::Integer(96), false, context)?;
    object.set(js_string!("F6"), JsValue::Integer(97), false, context)?;
    object.set(js_string!("F7"), JsValue::Integer(98), false, context)?;
    object.set(js_string!("F8"), JsValue::Integer(100), false, context)?;
    object.set(js_string!("F9"), JsValue::Integer(101), false, context)?;
    object.set(js_string!("FUNCTION"), JsValue::Integer(63), false, context)?;
    object.set(
        js_string!("LEFT_ARROW"),
        JsValue::Integer(123),
        false,
        context,
    )?;
    object.set(
        js_string!("META_LEFT"),
        JsValue::Integer(55),
        false,
        context,
    )?;
    object.set(
        js_string!("META_RIGHT"),
        JsValue::Integer(54),
        false,
        context,
    )?;
    object.set(js_string!("RETURN"), JsValue::Integer(36), false, context)?;
    object.set(
        js_string!("RIGHT_ARROW"),
        JsValue::Integer(124),
        false,
        context,
    )?;
    object.set(
        js_string!("SHIFT_LEFT"),
        JsValue::Integer(56),
        false,
        context,
    )?;
    object.set(
        js_string!("SHIFT_RIGHT"),
        JsValue::Integer(60),
        false,
        context,
    )?;
    object.set(js_string!("SPACE"), JsValue::Integer(49), false, context)?;
    object.set(js_string!("TAB"), JsValue::Integer(48), false, context)?;
    object.set(
        js_string!("UP_ARROW"),
        JsValue::Integer(126),
        false,
        context,
    )?;
    object.set(
        js_string!("BACK_QUOTE"),
        JsValue::Integer(50),
        false,
        context,
    )?;
    object.set(js_string!("NUM1"), JsValue::Integer(18), false, context)?;
    object.set(js_string!("NUM2"), JsValue::Integer(19), false, context)?;
    object.set(js_string!("NUM3"), JsValue::Integer(20), false, context)?;
    object.set(js_string!("NUM4"), JsValue::Integer(21), false, context)?;
    object.set(js_string!("NUM5"), JsValue::Integer(23), false, context)?;
    object.set(js_string!("NUM6"), JsValue::Integer(22), false, context)?;
    object.set(js_string!("NUM7"), JsValue::Integer(26), false, context)?;
    object.set(js_string!("NUM8"), JsValue::Integer(28), false, context)?;
    object.set(js_string!("NUM9"), JsValue::Integer(25), false, context)?;
    object.set(js_string!("NUM0"), JsValue::Integer(29), false, context)?;
    object.set(js_string!("MINUS"), JsValue::Integer(27), false, context)?;
    object.set(js_string!("EQUAL"), JsValue::Integer(24), false, context)?;
    object.set(js_string!("Q"), JsValue::Integer(12), false, context)?;
    object.set(js_string!("W"), JsValue::Integer(13), false, context)?;
    object.set(js_string!("E"), JsValue::Integer(14), false, context)?;
    object.set(js_string!("R"), JsValue::Integer(15), false, context)?;
    object.set(js_string!("T"), JsValue::Integer(17), false, context)?;
    object.set(js_string!("Y"), JsValue::Integer(16), false, context)?;
    object.set(js_string!("U"), JsValue::Integer(32), false, context)?;
    object.set(js_string!("I"), JsValue::Integer(34), false, context)?;
    object.set(js_string!("O"), JsValue::Integer(31), false, context)?;
    object.set(js_string!("P"), JsValue::Integer(35), false, context)?;
    object.set(
        js_string!("LEFT_BRACKET"),
        JsValue::Integer(33),
        false,
        context,
    )?;
    object.set(
        js_string!("RIGHT_BRACKET"),
        JsValue::Integer(30),
        false,
        context,
    )?;
    object.set(js_string!("A"), JsValue::Integer(0), false, context)?;
    object.set(js_string!("S"), JsValue::Integer(1), false, context)?;
    object.set(js_string!("D"), JsValue::Integer(2), false, context)?;
    object.set(js_string!("F"), JsValue::Integer(3), false, context)?;
    object.set(js_string!("G"), JsValue::Integer(5), false, context)?;
    object.set(js_string!("H"), JsValue::Integer(4), false, context)?;
    object.set(js_string!("J"), JsValue::Integer(38), false, context)?;
    object.set(js_string!("K"), JsValue::Integer(40), false, context)?;
    object.set(js_string!("L"), JsValue::Integer(37), false, context)?;
    object.set(
        js_string!("SEMI_COLON"),
        JsValue::Integer(41),
        false,
        context,
    )?;
    object.set(js_string!("QUOTE"), JsValue::Integer(39), false, context)?;
    object.set(
        js_string!("BACK_SLASH"),
        JsValue::Integer(42),
        false,
        context,
    )?;
    object.set(js_string!("Z"), JsValue::Integer(6), false, context)?;
    object.set(js_string!("X"), JsValue::Integer(7), false, context)?;
    object.set(js_string!("C"), JsValue::Integer(8), false, context)?;
    object.set(js_string!("V"), JsValue::Integer(9), false, context)?;
    object.set(js_string!("B"), JsValue::Integer(11), false, context)?;
    object.set(js_string!("N"), JsValue::Integer(45), false, context)?;
    object.set(js_string!("M"), JsValue::Integer(46), false, context)?;
    object.set(js_string!("COMMA"), JsValue::Integer(43), false, context)?;
    object.set(js_string!("DOT"), JsValue::Integer(47), false, context)?;
    object.set(js_string!("SLASH"), JsValue::Integer(44), false, context)?;
    Ok(object)
}

impl Class for JsKeyCode {
    const NAME: &'static str = "KeyCode";
    const LENGTH: usize = 1;

    fn constructor(_this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<Self> {
        Ok(JsKeyCode {})
    }

    /// Here is where the class is initialized, to be inserted into the global object.
    fn init(class: &mut ClassBuilder) -> JsResult<()> {
        class.static_property("ALT", 58, Attribute::READONLY);
        class.static_property("ALT_GR", 61, Attribute::READONLY);
        class.static_property("BACKSPACE", 51, Attribute::READONLY);
        class.static_property("CAPS_LOCK", 57, Attribute::READONLY);
        class.static_property("CONTROL_LEFT", 59, Attribute::READONLY);
        class.static_property("CONTROL_RIGHT", 62, Attribute::READONLY);
        class.static_property("DOWN_ARROW", 125, Attribute::READONLY);
        class.static_property("ESCAPE", 53, Attribute::READONLY);
        class.static_property("F1", 122, Attribute::READONLY);
        class.static_property("F10", 109, Attribute::READONLY);
        class.static_property("F11", 103, Attribute::READONLY);
        class.static_property("F12", 111, Attribute::READONLY);
        class.static_property("F2", 120, Attribute::READONLY);
        class.static_property("F3", 99, Attribute::READONLY);
        class.static_property("F4", 118, Attribute::READONLY);
        class.static_property("F5", 96, Attribute::READONLY);
        class.static_property("F6", 97, Attribute::READONLY);
        class.static_property("F7", 98, Attribute::READONLY);
        class.static_property("F8", 100, Attribute::READONLY);
        class.static_property("F9", 101, Attribute::READONLY);
        class.static_property("FUNCTION", 63, Attribute::READONLY);
        class.static_property("LEFT_ARROW", 123, Attribute::READONLY);
        class.static_property("META_LEFT", 55, Attribute::READONLY);
        class.static_property("META_RIGHT", 54, Attribute::READONLY);
        class.static_property("RETURN", 36, Attribute::READONLY);
        class.static_property("RIGHT_ARROW", 124, Attribute::READONLY);
        class.static_property("SHIFT_LEFT", 56, Attribute::READONLY);
        class.static_property("SHIFT_RIGHT", 60, Attribute::READONLY);
        class.static_property("SPACE", 49, Attribute::READONLY);
        class.static_property("TAB", 48, Attribute::READONLY);
        class.static_property("UP_ARROW", 126, Attribute::READONLY);
        class.static_property("BACK_QUOTE", 50, Attribute::READONLY);
        class.static_property("NUM1", 18, Attribute::READONLY);
        class.static_property("NUM2", 19, Attribute::READONLY);
        class.static_property("NUM3", 20, Attribute::READONLY);
        class.static_property("NUM4", 21, Attribute::READONLY);
        class.static_property("NUM5", 23, Attribute::READONLY);
        class.static_property("NUM6", 22, Attribute::READONLY);
        class.static_property("NUM7", 26, Attribute::READONLY);
        class.static_property("NUM8", 28, Attribute::READONLY);
        class.static_property("NUM9", 25, Attribute::READONLY);
        class.static_property("NUM0", 29, Attribute::READONLY);
        class.static_property("MINUS", 27, Attribute::READONLY);
        class.static_property("EQUAL", 24, Attribute::READONLY);
        class.static_property("Q", 12, Attribute::READONLY);
        class.static_property("W", 13, Attribute::READONLY);
        class.static_property("E", 14, Attribute::READONLY);
        class.static_property("R", 15, Attribute::READONLY);
        class.static_property("T", 17, Attribute::READONLY);
        class.static_property("Y", 16, Attribute::READONLY);
        class.static_property("U", 32, Attribute::READONLY);
        class.static_property("I", 34, Attribute::READONLY);
        class.static_property("O", 31, Attribute::READONLY);
        class.static_property("P", 35, Attribute::READONLY);
        class.static_property("LEFT_BRACKET", 33, Attribute::READONLY);
        class.static_property("RIGHT_BRACKET", 30, Attribute::READONLY);
        class.static_property("A", 0, Attribute::READONLY);
        class.static_property("S", 1, Attribute::READONLY);
        class.static_property("D", 2, Attribute::READONLY);
        class.static_property("F", 3, Attribute::READONLY);
        class.static_property("G", 5, Attribute::READONLY);
        class.static_property("H", 4, Attribute::READONLY);
        class.static_property("J", 38, Attribute::READONLY);
        class.static_property("K", 40, Attribute::READONLY);
        class.static_property("L", 37, Attribute::READONLY);
        class.static_property("SEMI_COLON", 41, Attribute::READONLY);
        class.static_property("QUOTE", 39, Attribute::READONLY);
        class.static_property("BACK_SLASH", 42, Attribute::READONLY);
        class.static_property("Z", 6, Attribute::READONLY);
        class.static_property("X", 7, Attribute::READONLY);
        class.static_property("C", 8, Attribute::READONLY);
        class.static_property("V", 9, Attribute::READONLY);
        class.static_property("B", 11, Attribute::READONLY);
        class.static_property("N", 45, Attribute::READONLY);
        class.static_property("M", 46, Attribute::READONLY);
        class.static_property("COMMA", 43, Attribute::READONLY);
        class.static_property("DOT", 47, Attribute::READONLY);
        class.static_property("SLASH", 44, Attribute::READONLY);

        Ok(())
    }
}
