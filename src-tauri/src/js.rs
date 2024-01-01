use std::collections::HashMap;
use std::fmt::{Debug};
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventField_kCGKeyboardEventKeycode, CGEventFlags_kCGEventFlagMaskNonCoalesced, CGEventGetFlags, CGEventGetIntegerValueField, CGEventRef, CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp};
use boa_engine::{Context, js_string, JsError, JsObject, JsResult, JsValue, NativeFunction, Source};

use boa_engine::native_function::NativeFunctionPointer;
use boa_engine::object::builtins::{JsArray, JsFunction, JsMap};
use boa_engine::property::{Attribute, PropertyKey};
use boa_engine::value::TryFromJs;
use boa_gc::{Finalize, GcRefCell, Trace};
use boa_runtime::Console;
use crate::app_config::AppConfig;
use crate::event::{event_type};

use crate::js_builtin::JsBuiltin;
use crate::js_hotkey::JsHotKey;

pub struct JS<'a> {
    context: Context<'a>,
}
impl JS<'_> {
    pub fn new() -> anyhow::Result<Self> {
        let mut context = Context::default();

        let mut js = JS { context, };
        js.init_console()?;
        js.init_hotkey()?;
        js.register_constants()?;
        js.register_builtin_functions()?;
        js.load_driver()?;
        Ok(js)
    }

    fn init_console(&mut self) -> anyhow::Result<()>{
        // expose `console` object
        let console = Console::init(&mut self.context);
        if let Err(err) = self.context
            .register_global_property(js_string!(Console::NAME), console, Attribute::all()) {
            return Err(anyhow!("Cannot register `console` object: {:?}", err));
        }
        Ok(())
    }

    fn init_hotkey(&mut self) -> anyhow::Result<()>{
        // expose `HotKey` object
        if let Err(err) = self.context.register_global_class::<JsHotKey>() {
            return Err(anyhow!("Cannot register `HotKey` object: {:?}", err));
        }
        Ok(())
    }

    fn register_constants(&mut self) -> anyhow::Result<()> {
        self.register_constant("kCGEventKeyDown", CGEventType_kCGEventKeyDown)?;
        self.register_constant("kCGEventKeyUp", CGEventType_kCGEventKeyUp)?;
        self.register_constant("kCGEventFlagsChanged", CGEventType_kCGEventFlagsChanged)?;
        self.register_constant("kCGKeyboardEventKeycode", CGEventField_kCGKeyboardEventKeycode)?;
        self.register_constant("kCGEventFlagMaskNonCoalesced", CGEventFlags_kCGEventFlagMaskNonCoalesced)?;
        Ok(())
    }

     fn register_constant<K, V>(&mut self, key: K, value: V) -> anyhow::Result<()>
         where
             K: Into<PropertyKey> + Debug + Copy,
             V: Into<JsValue>,
     {
        if let Err(err) = self.context.register_global_property(key,  value, Attribute::READONLY) {
            return Err(anyhow!("Cannot register constant: {:?}, {:?}", key, err));
        }
        Ok(())
    }

    fn register_builtin_functions(&mut self) -> anyhow::Result<()> {
        fn register(context: &mut Context, name: &str, fn_ptr: NativeFunctionPointer) -> anyhow::Result<()> {
            if let Err(err) = context.register_global_callable(
                name,
                1,
                NativeFunction::from_fn_ptr(fn_ptr)
            ) {
                return Err(anyhow!("Cannot register `{}` function: {:?}", name, err));
            }

            Ok(())
        }

        register(&mut self.context, "sendFlagsChangedEvent", JsBuiltin::send_flags_changed_event)?;
        register(&mut self.context, "sendKeyboardEvent", JsBuiltin::send_keyboard_event)?;
        register(&mut self.context, "loadAppConfigJson", JsBuiltin::load_app_config_json)?;
        Ok(())
    }

    pub fn eval(&mut self, src: String) -> anyhow::Result<JsValue> {
        return match self.context.eval(Source::from_bytes(&src)) {
            Ok(value) => { Ok(value) }
            Err(err) => {
                Err(anyhow!("Cannot execute javascript code: {:?}", err))
            }
        }
    }

    // Call this method when key/mouse event was received.
    // This method calls JS handlers.
    pub fn send_event(&mut self, cg_event_type: CGEventType, cg_event_ref: CGEventRef) -> anyhow::Result<bool> {
        let invoke_event = self.context.global_object().get("$$invokeEvent", &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$invokeEvent: {:?}", err))?;
        let invoke_event = JsFunction::try_from_js(&invoke_event, &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$invokeEvent as JsFunction: {:?}", err))?;


        let key_event = self.build_key_event(cg_event_type, cg_event_ref)?;
        let result = invoke_event.call(&JsValue::undefined(), &[JsValue::from(key_event)], &mut self.context)
            .map_err(|err| anyhow!("Cannot call $$invokeEvent as JsFunction: {:?}", err))?;
        let result = result.as_boolean()
            .unwrap_or(true);
        Ok(result)
    }

    fn build_config(&mut self, id_rs: &String, config: &AppConfig, config_schema: JsMap) -> JsResult<JsMap> {
        let result = JsMap::new(&mut self.context);

        let user_config = if let Some(plugin_configs) = &config.plugins {
            if let Some(user_config) = plugin_configs.get(id_rs) {
                user_config.clone()
            } else {
                HashMap::default()
            }
        } else {
            HashMap::default()
        };

        let pf = config_schema.keys(&mut self.context)?;
        loop {
            let key = pf.next(&mut self.context)?;
            if key.is_null_or_undefined() {
                break;
            }

            let value = config_schema.get(key, &mut self.context)?;
            let schema_for_item = JsMap::try_from_js(&value, &mut self.context)?;

            let name = schema_for_item.get(js_string!("name"), &mut self.context)?;
            let type_rs = schema_for_item.get(js_string!("type"), &mut self.context)?
                .to_string(&mut self.context)?
                .to_std_string()
                .map_err(|err| JsError::from_opaque(format!("cannot convert string: {}", err).into()))?;
            let default_rs = schema_for_item.get(js_string!("default"), &mut self.context)?
                .to_string(&mut self.context)?
                .to_std_string()
                .map_err(|err| JsError::from_opaque(format!("cannot convert string: {}", err).into()))?;

            match type_rs.as_str() {
                "hotkey" => {
                    // fallback to default value
                    let hotkey = if let Some(user_config) = user_config.get(name.to_string(&mut self.context)?
                        .to_std_string()
                        .map_err(|err| JsError::from_opaque(format!("cannot convert string: {}", err).into()))?
                        .as_str()) {
                        user_config.as_str()
                    } else {
                        // use default values...
                        default_rs.as_str()
                    };
                    // let hotkey = HotKey::from_str(default_rs.as_str())?;
                    result.set(name.clone(), /* hotkey */hotkey, &mut self.context)?;
                }
                _ => {
                    return Err(JsError::from_opaque(format!("Unknown type: {}", type_rs).into()));
                }
            }

            // if value is {"name": "hotkey", "type": "hotkey", "default": "C-t"}
        }
        Ok(result)
    }

    fn build_key_event(&mut self, cg_event_type: CGEventType, cg_event_ref: CGEventRef) -> anyhow::Result<JsObject> {
        let key_event = JsObject::with_object_proto(self.context.intrinsics());
        fn set<K, V>(js: &mut JS<'_>, key_event: &JsObject, key: K, value: V) -> anyhow::Result<()>
            where
                K: Into<PropertyKey>,
                V: Into<JsValue>,
        {
            if let Err(err) = key_event.set(key,
                                            value,
                                            false, &mut js.context) {
                return Err(anyhow!("Cannot set name: {:?}", err));
            }
            Ok(())
        }

        unsafe {
            set(self, &key_event, js_string!("type"), js_string!(event_type(cg_event_type)))?;

            let code = CGEventGetIntegerValueField(cg_event_ref, CGEventField_kCGKeyboardEventKeycode);
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
}
