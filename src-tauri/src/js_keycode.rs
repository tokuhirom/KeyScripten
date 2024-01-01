use boa_engine::{js_string, Context, JsObject, JsResult, JsValue};

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
