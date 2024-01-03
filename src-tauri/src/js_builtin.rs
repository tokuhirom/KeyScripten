use crate::app_config::AppConfig;
use crate::send::{send_flags_changed_event, send_keyboard_event};
use apple_sys::CoreGraphics::{CGEventFlags, CGKeyCode};
use boa_engine::{Context, JsArgs, JsError, JsNativeError, JsResult, JsString, JsValue};

pub struct JsBuiltin {}

impl JsBuiltin {
    pub fn send_flags_changed_event(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context<'_>,
    ) -> JsResult<JsValue> {
        let flags: &JsValue = args.get_or_undefined(0);

        if let Err(err) = send_flags_changed_event(flags.to_i32(context).unwrap() as CGEventFlags) {
            return Err(JsNativeError::typ()
                .with_message(format!("Cannot run send_flags_changed_event: {:?}", err))
                .into());
        }

        Ok(JsValue::undefined())
    }

    pub fn send_keyboard_event(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context<'_>,
    ) -> JsResult<JsValue> {
        let keycode: &JsValue = args.get_or_undefined(0);
        let flags: &JsValue = args.get_or_undefined(1);
        let pressed: &JsValue = args.get_or_undefined(2);

        if let Err(err) = send_keyboard_event(
            keycode.to_i32(context).unwrap() as CGKeyCode,
            flags.to_i32(context).unwrap() as CGEventFlags,
            pressed.to_boolean(),
        ) {
            return Err(JsNativeError::typ()
                .with_message(format!("Cannot run send_keyboard_event: {:?}", err))
                .into());
        }

        Ok(JsValue::undefined())
    }

    pub fn load_app_config_json(
        _this: &JsValue,
        _args: &[JsValue],
        _context: &mut Context<'_>,
    ) -> JsResult<JsValue> {
        match AppConfig::load() {
            Ok(config) => match serde_json::to_string(&config) {
                Ok(json) => {
                    return Ok(JsValue::String(JsString::from(json.as_str())));
                }
                Err(err) => Err(JsError::from_opaque(
                    format!("Cannot make json: {:?}", err).into(),
                )),
            },
            Err(err) => Err(JsError::from_opaque(
                format!("Cannot load configuration: {:?}", err).into(),
            )),
        }
    }
}
