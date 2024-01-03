use anyhow::anyhow;
use apple_sys::CoreGraphics::{
    CGEventField_kCGKeyboardEventKeycode, CGEventFlags_kCGEventFlagMaskNonCoalesced,
    CGEventGetFlags, CGEventGetIntegerValueField, CGEventRef, CGEventType,
    CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp,
};
use boa_engine::{js_string, Context, JsObject, JsValue, NativeFunction, Source};
use std::collections::HashMap;
use std::fmt::Debug;

use boa_engine::native_function::NativeFunctionPointer;
use boa_engine::object::builtins::JsFunction;
use boa_engine::property::{Attribute, PropertyKey};
use boa_engine::value::TryFromJs;

use crate::event::event_type;
use boa_runtime::Console;
use serde::{Deserialize, Serialize};

use crate::js_builtin::JsBuiltin;
use crate::js_hotkey::JsHotKey;
use crate::js_keycode::build_keycode;

pub struct JS<'a> {
    context: Context<'a>,
}
impl JS<'_> {
    pub fn new() -> anyhow::Result<Self> {
        let context = Context::default();

        let mut js = JS { context };
        js.init_console()?;
        js.init_hotkey()?;
        js.init_keycode()?;
        js.register_constants()?;
        js.register_builtin_functions()?;
        js.load_driver()?;
        js.load_bundled()?;
        Ok(js)
    }

    fn init_console(&mut self) -> anyhow::Result<()> {
        // expose `console` object
        let console = Console::init(&mut self.context);
        if let Err(err) = self.context.register_global_property(
            js_string!(Console::NAME),
            console,
            Attribute::all(),
        ) {
            return Err(anyhow!("Cannot register `console` object: {:?}", err));
        }
        Ok(())
    }

    fn init_hotkey(&mut self) -> anyhow::Result<()> {
        // expose `HotKey` object
        if let Err(err) = self.context.register_global_class::<JsHotKey>() {
            return Err(anyhow!("Cannot register `HotKey` object: {:?}", err));
        }
        Ok(())
    }

    fn init_keycode(&mut self) -> anyhow::Result<()> {
        let keycode = build_keycode(&mut self.context)
            .map_err(|err| anyhow!("Cannot build keycode object: {:?}", err))?;
        self.register_constant("Key", keycode)?;
        Ok(())
    }

    fn register_constants(&mut self) -> anyhow::Result<()> {
        self.register_constant("kCGEventKeyDown", CGEventType_kCGEventKeyDown)?;
        self.register_constant("kCGEventKeyUp", CGEventType_kCGEventKeyUp)?;
        self.register_constant("kCGEventFlagsChanged", CGEventType_kCGEventFlagsChanged)?;
        self.register_constant(
            "kCGKeyboardEventKeycode",
            CGEventField_kCGKeyboardEventKeycode,
        )?;
        self.register_constant(
            "kCGEventFlagMaskNonCoalesced",
            CGEventFlags_kCGEventFlagMaskNonCoalesced,
        )?;
        Ok(())
    }

    fn register_constant<K, V>(&mut self, key: K, value: V) -> anyhow::Result<()>
    where
        K: Into<PropertyKey> + Debug + Copy,
        V: Into<JsValue>,
    {
        if let Err(err) = self
            .context
            .register_global_property(key, value, Attribute::READONLY)
        {
            return Err(anyhow!("Cannot register constant: {:?}, {:?}", key, err));
        }
        Ok(())
    }

    fn register_builtin_functions(&mut self) -> anyhow::Result<()> {
        fn register(
            context: &mut Context,
            name: &str,
            fn_ptr: NativeFunctionPointer,
        ) -> anyhow::Result<()> {
            if let Err(err) =
                context.register_global_callable(name, 1, NativeFunction::from_fn_ptr(fn_ptr))
            {
                return Err(anyhow!("Cannot register `{}` function: {:?}", name, err));
            }

            Ok(())
        }

        register(
            &mut self.context,
            "sendFlagsChangedEvent",
            JsBuiltin::send_flags_changed_event,
        )?;
        register(
            &mut self.context,
            "sendKeyboardEvent",
            JsBuiltin::send_keyboard_event,
        )?;
        register(
            &mut self.context,
            "loadAppConfigJson",
            JsBuiltin::load_app_config_json,
        )?;
        Ok(())
    }

    pub fn eval(&mut self, src: String) -> anyhow::Result<JsValue> {
        return match self.context.eval(Source::from_bytes(&src)) {
            Ok(value) => Ok(value),
            Err(err) => Err(anyhow!("Cannot execute javascript code: {:?}", err)),
        };
    }

    // Call this method when key/mouse event was received.
    // This method calls JS handlers.
    pub fn send_event(
        &mut self,
        cg_event_type: CGEventType,
        cg_event_ref: CGEventRef,
    ) -> anyhow::Result<bool> {
        let invoke_event = self
            .context
            .global_object()
            .get("$$invokeEvent", &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$invokeEvent: {:?}", err))?;
        let invoke_event = JsFunction::try_from_js(&invoke_event, &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$invokeEvent as JsFunction: {:?}", err))?;

        let key_event = self.build_key_event(cg_event_type, cg_event_ref)?;
        let result = invoke_event
            .call(
                &JsValue::undefined(),
                &[JsValue::from(key_event)],
                &mut self.context,
            )
            .map_err(|err| anyhow!("Cannot call $$invokeEvent as JsFunction: {:?}", err))?;
        let result = result.as_boolean().unwrap_or(true);
        Ok(result)
    }

    fn build_key_event(
        &mut self,
        cg_event_type: CGEventType,
        cg_event_ref: CGEventRef,
    ) -> anyhow::Result<JsObject> {
        let key_event = JsObject::with_object_proto(self.context.intrinsics());
        fn set<K, V>(js: &mut JS<'_>, key_event: &JsObject, key: K, value: V) -> anyhow::Result<()>
        where
            K: Into<PropertyKey>,
            V: Into<JsValue>,
        {
            if let Err(err) = key_event.set(key, value, false, &mut js.context) {
                return Err(anyhow!("Cannot set name: {:?}", err));
            }
            Ok(())
        }

        unsafe {
            set(
                self,
                &key_event,
                js_string!("type"),
                js_string!(event_type(cg_event_type)),
            )?;

            let code =
                CGEventGetIntegerValueField(cg_event_ref, CGEventField_kCGKeyboardEventKeycode);
            set(self, &key_event, js_string!("keycode"), JsValue::from(code))?;

            if cg_event_type == CGEventType_kCGEventFlagsChanged {
                let flags = CGEventGetFlags(cg_event_ref);
                set(self, &key_event, js_string!("flags"), JsValue::from(flags))?;
            }
        }

        Ok(key_event)
    }

    fn load_driver(&mut self) -> anyhow::Result<JsValue> {
        let driver_src = include_str!("../js/driver.js");
        self.eval(driver_src.to_string())
    }

    fn load_bundled(&mut self) -> anyhow::Result<JsValue> {
        let src = include_str!("../js/dynamic-macro.js");
        self.eval(src.to_string())
    }

    #[allow(dead_code)]
    pub fn get_config_schema(&mut self) -> anyhow::Result<ConfigSchemaList> {
        let get_config_schema = self
            .context
            .global_object()
            .get("$$getConfigSchema", &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$getConfigSchema: {:?}", err))?;
        let get_config_schema = JsFunction::try_from_js(&get_config_schema, &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$getConfigSchema as JsFunction: {:?}", err))?;

        let result = get_config_schema
            .call(&JsValue::undefined(), &[], &mut self.context)
            .map_err(|err| anyhow!("Cannot call $$getConfigSchema as JsFunction: {:?}", err))?;
        let result = result
            .to_string(&mut self.context)
            .map_err(|err| anyhow!("Cannot get result from $$getConfigSchema: {:?}", err))?;
        let result = result.to_std_string_escaped();
        let result: ConfigSchemaList = serde_json::from_str(&result)?;
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigSchemaList {
    plugins: Vec<ConfigSchema>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigSchema {
    id: String,
    config: Vec<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_keycode() -> anyhow::Result<()> {
        let mut js = JS::new()?;
        let schema = js.get_config_schema()?;
        log::info!("schema={:?}", schema);
        Ok(())
    }

    #[test]
    fn test_eval() -> anyhow::Result<()> {
        let mut js = JS::new()?;
        let value = js.eval("3+4".to_string())?;
        let got = value.to_u32(&mut js.context).unwrap();
        assert_eq!(got, 7);
        Ok(())
    }
}
