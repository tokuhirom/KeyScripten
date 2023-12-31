use apple_sys::CoreGraphics::{CGEventFlags, CGKeyCode};
use boa_engine::{Context, js_string, JsArgs, JsNativeError, JsResult, JsValue};
use boa_gc::{GcRef, GcRefCell};
use crate::handler::matches_hotkey_string;
use crate::js::BigStruct;
use crate::send::{send_flags_changed_event, send_keyboard_event};
use crate::shortcut::parse_shortcut;

pub struct JsBuiltin {

}

impl JsBuiltin {
    pub fn register_plugin(_this: &JsValue, args: &[JsValue], captures: &GcRefCell<BigStruct>, context: &mut Context<'_>) -> JsResult<JsValue> {
        let id: &JsValue = args.get(0).unwrap();
        let _name = args.get(1).unwrap();
        let callback = args.get(2).unwrap();
        let _config_schema = args.get(3).unwrap();

        let mut captures = captures.borrow_mut();
        let BigStruct { id_list, callbacks } = &mut *captures;

        // push id to the array
        id_list.push(id.clone(), context).unwrap();

        log::info!("id_list={:?}, {}", id_list, id_list.length(context).unwrap());

        callbacks.set(id.clone(), callback.clone(), context).unwrap();

        Ok(JsValue::undefined())
    }

    pub fn matches_hotkey_string(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        let flags: &JsValue = args.get_or_undefined(0);
        let keycode = args.get_or_undefined(1);
        let shortcut = args.get_or_undefined(2);
        let shortcut = shortcut.as_string().unwrap().to_std_string().unwrap();

        match parse_shortcut(shortcut.as_str()) { // TODO cache? config をパースしたタイミングで、ショートカットのパースもしておくべき
            Ok(shortcut) => {
                let result = matches_hotkey_string(flags.to_i32(context).unwrap() as CGEventFlags,
                                                   keycode.to_i32(context).unwrap() as CGKeyCode,
                                                   &shortcut);

                Ok(JsValue::from(result))
            }
            Err(err) => {
                Err(JsNativeError::typ()
                    .with_message(format!("Cannot run parse_shortcut: {:?}", err))
                    .into())
            }
        }
    }

    pub fn send_flags_changed_event(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        let flags: &JsValue = args.get_or_undefined(0);

        if let Err(err) = send_flags_changed_event(flags.to_i32(context).unwrap() as CGEventFlags) {
            return Err(JsNativeError::typ()
                .with_message(format!("Cannot run send_flags_changed_event: {:?}", err))
                .into());
        }

        Ok(JsValue::undefined())
    }

     pub fn send_keyboard_event(_this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
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
                 .into())
         }

         Ok(JsValue::undefined())
     }

}
