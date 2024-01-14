use crate::hotkey::HotKey;
use boa_engine::class::{Class, ClassBuilder};
use boa_engine::{Context, JsArgs, JsError, JsResult, JsValue, NativeFunction};
use boa_gc::{Finalize, Trace};

#[derive(Debug, Trace, Finalize)]
pub struct JsHotKey {
    hotkey: HotKey,
}

impl JsHotKey {
    fn matches(this: &JsValue, args: &[JsValue], context: &mut Context<'_>) -> JsResult<JsValue> {
        let Some(this) = this.as_object().and_then(|obj| obj.downcast_ref::<Self>()) else {
            return Err(JsError::from_opaque(
                "the 'this' object is not a JsHotkey".into(),
            ));
        };

        let Some(flags) = args.first() else {
            return Err(JsError::from_opaque(
                "first argument of the JsHotKey.matches should be flags.".into(),
            ));
        };
        let flags = match flags.to_u32(context) {
            Ok(flags) => flags,
            Err(err) => {
                return Err(JsError::from_opaque(
                    format!(
                        "first argument of the JsHotKey.matches should be int: {:?}",
                        err
                    )
                    .into(),
                ));
            }
        };
        let flags: u64 = flags.into();

        let Some(keycode) = args.get(1) else {
            return Err(JsError::from_opaque(
                "second argument of the JsHotKey.matches should be keycode.".into(),
            ));
        };
        let keycode = match keycode.to_uint16(context) {
            Ok(keycode) => keycode,
            Err(err) => {
                return Err(JsError::from_opaque(
                    format!(
                        "second argument of the JsHotKey.matches should be u16(keycode): {:?}.",
                        err
                    )
                    .into(),
                ));
            }
        };

        let result = this.hotkey.matches(flags, keycode);

        Ok(JsValue::Boolean(result))
    }
}

impl Class for JsHotKey {
    const NAME: &'static str = "HotKey";
    const LENGTH: usize = 1;

    // This is what is called when we construct a `HotKey` with the expression `new HotKey()`.
    fn constructor(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<Self> {
        let string_notation = args.get_or_undefined(0).to_string(context)?;
        let string_notation = string_notation.to_std_string_escaped();

        let hotkey = HotKey::from_str(string_notation.as_str()).map_err(|err| {
            JsError::from_opaque(format!("Cannot parse shortcut key: {:?}", err).into())
        })?;

        Ok(JsHotKey { hotkey })
    }

    /// Here is where the class is initialized, to be inserted into the global object.
    fn init(class: &mut ClassBuilder) -> JsResult<()> {
        class.method("matches", 0, NativeFunction::from_fn_ptr(Self::matches));

        Ok(())
    }
}
