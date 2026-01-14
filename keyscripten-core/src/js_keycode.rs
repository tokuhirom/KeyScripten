use boa_engine::{js_string, Context, JsObject, JsResult, JsValue};

pub fn build_keycode(context: &mut Context) -> JsResult<JsObject> {
    let object = JsObject::with_null_proto();
    object.set(js_string!("ALT"), JsValue::new(58), false, context)?;
    object.set(js_string!("ALT_GR"), JsValue::new(61), false, context)?;
    object.set(
        js_string!("BACKSPACE"),
        JsValue::new(51),
        false,
        context,
    )?;
    object.set(
        js_string!("CAPS_LOCK"),
        JsValue::new(57),
        false,
        context,
    )?;
    object.set(
        js_string!("CONTROL_LEFT"),
        JsValue::new(59),
        false,
        context,
    )?;
    object.set(
        js_string!("CONTROL_RIGHT"),
        JsValue::new(62),
        false,
        context,
    )?;
    object.set(
        js_string!("DOWN_ARROW"),
        JsValue::new(125),
        false,
        context,
    )?;
    object.set(js_string!("ESCAPE"), JsValue::new(53), false, context)?;
    object.set(js_string!("F1"), JsValue::new(122), false, context)?;
    object.set(js_string!("F10"), JsValue::new(109), false, context)?;
    object.set(js_string!("F11"), JsValue::new(103), false, context)?;
    object.set(js_string!("F12"), JsValue::new(111), false, context)?;
    object.set(js_string!("F2"), JsValue::new(120), false, context)?;
    object.set(js_string!("F3"), JsValue::new(99), false, context)?;
    object.set(js_string!("F4"), JsValue::new(118), false, context)?;
    object.set(js_string!("F5"), JsValue::new(96), false, context)?;
    object.set(js_string!("F6"), JsValue::new(97), false, context)?;
    object.set(js_string!("F7"), JsValue::new(98), false, context)?;
    object.set(js_string!("F8"), JsValue::new(100), false, context)?;
    object.set(js_string!("F9"), JsValue::new(101), false, context)?;
    object.set(js_string!("FUNCTION"), JsValue::new(63), false, context)?;
    object.set(
        js_string!("LEFT_ARROW"),
        JsValue::new(123),
        false,
        context,
    )?;
    object.set(
        js_string!("META_LEFT"),
        JsValue::new(55),
        false,
        context,
    )?;
    object.set(
        js_string!("META_RIGHT"),
        JsValue::new(54),
        false,
        context,
    )?;
    object.set(js_string!("RETURN"), JsValue::new(36), false, context)?;
    object.set(
        js_string!("RIGHT_ARROW"),
        JsValue::new(124),
        false,
        context,
    )?;
    object.set(
        js_string!("SHIFT_LEFT"),
        JsValue::new(56),
        false,
        context,
    )?;
    object.set(
        js_string!("SHIFT_RIGHT"),
        JsValue::new(60),
        false,
        context,
    )?;
    object.set(js_string!("SPACE"), JsValue::new(49), false, context)?;
    object.set(js_string!("TAB"), JsValue::new(48), false, context)?;
    object.set(
        js_string!("UP_ARROW"),
        JsValue::new(126),
        false,
        context,
    )?;
    object.set(
        js_string!("BACK_QUOTE"),
        JsValue::new(50),
        false,
        context,
    )?;
    object.set(js_string!("NUM1"), JsValue::new(18), false, context)?;
    object.set(js_string!("NUM2"), JsValue::new(19), false, context)?;
    object.set(js_string!("NUM3"), JsValue::new(20), false, context)?;
    object.set(js_string!("NUM4"), JsValue::new(21), false, context)?;
    object.set(js_string!("NUM5"), JsValue::new(23), false, context)?;
    object.set(js_string!("NUM6"), JsValue::new(22), false, context)?;
    object.set(js_string!("NUM7"), JsValue::new(26), false, context)?;
    object.set(js_string!("NUM8"), JsValue::new(28), false, context)?;
    object.set(js_string!("NUM9"), JsValue::new(25), false, context)?;
    object.set(js_string!("NUM0"), JsValue::new(29), false, context)?;
    object.set(js_string!("MINUS"), JsValue::new(27), false, context)?;
    object.set(js_string!("EQUAL"), JsValue::new(24), false, context)?;
    object.set(js_string!("Q"), JsValue::new(12), false, context)?;
    object.set(js_string!("W"), JsValue::new(13), false, context)?;
    object.set(js_string!("E"), JsValue::new(14), false, context)?;
    object.set(js_string!("R"), JsValue::new(15), false, context)?;
    object.set(js_string!("T"), JsValue::new(17), false, context)?;
    object.set(js_string!("Y"), JsValue::new(16), false, context)?;
    object.set(js_string!("U"), JsValue::new(32), false, context)?;
    object.set(js_string!("I"), JsValue::new(34), false, context)?;
    object.set(js_string!("O"), JsValue::new(31), false, context)?;
    object.set(js_string!("P"), JsValue::new(35), false, context)?;
    object.set(
        js_string!("LEFT_BRACKET"),
        JsValue::new(33),
        false,
        context,
    )?;
    object.set(
        js_string!("RIGHT_BRACKET"),
        JsValue::new(30),
        false,
        context,
    )?;
    object.set(js_string!("A"), JsValue::new(0), false, context)?;
    object.set(js_string!("S"), JsValue::new(1), false, context)?;
    object.set(js_string!("D"), JsValue::new(2), false, context)?;
    object.set(js_string!("F"), JsValue::new(3), false, context)?;
    object.set(js_string!("G"), JsValue::new(5), false, context)?;
    object.set(js_string!("H"), JsValue::new(4), false, context)?;
    object.set(js_string!("J"), JsValue::new(38), false, context)?;
    object.set(js_string!("K"), JsValue::new(40), false, context)?;
    object.set(js_string!("L"), JsValue::new(37), false, context)?;
    object.set(
        js_string!("SEMI_COLON"),
        JsValue::new(41),
        false,
        context,
    )?;
    object.set(js_string!("QUOTE"), JsValue::new(39), false, context)?;
    object.set(
        js_string!("BACK_SLASH"),
        JsValue::new(42),
        false,
        context,
    )?;
    object.set(js_string!("Z"), JsValue::new(6), false, context)?;
    object.set(js_string!("X"), JsValue::new(7), false, context)?;
    object.set(js_string!("C"), JsValue::new(8), false, context)?;
    object.set(js_string!("V"), JsValue::new(9), false, context)?;
    object.set(js_string!("B"), JsValue::new(11), false, context)?;
    object.set(js_string!("N"), JsValue::new(45), false, context)?;
    object.set(js_string!("M"), JsValue::new(46), false, context)?;
    object.set(js_string!("COMMA"), JsValue::new(43), false, context)?;
    object.set(js_string!("DOT"), JsValue::new(47), false, context)?;
    object.set(js_string!("SLASH"), JsValue::new(44), false, context)?;
    Ok(object)
}
