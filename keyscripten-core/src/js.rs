use anyhow::anyhow;
use apple_sys::CoreGraphics::{
    CGEventField_kCGKeyboardEventKeycode, CGEventFlags_kCGEventFlagMaskAlphaShift,
    CGEventFlags_kCGEventFlagMaskAlternate, CGEventFlags_kCGEventFlagMaskCommand,
    CGEventFlags_kCGEventFlagMaskControl, CGEventFlags_kCGEventFlagMaskHelp,
    CGEventFlags_kCGEventFlagMaskNonCoalesced, CGEventFlags_kCGEventFlagMaskNumericPad,
    CGEventFlags_kCGEventFlagMaskSecondaryFn, CGEventFlags_kCGEventFlagMaskShift, CGEventRef,
    CGEventType, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown,
    CGEventType_kCGEventKeyUp,
};
use boa_engine::{js_string, Context, JsObject, JsValue, NativeFunction, Source};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Arc, RwLock};

use boa_engine::native_function::NativeFunctionPointer;
use boa_engine::object::builtins::JsFunction;
use boa_engine::property::{Attribute, PropertyKey};
use boa_engine::value::TryFromJs;

use crate::event::Event;
use crate::js_console::Console;
use serde::{Deserialize, Serialize};

use crate::js_builtin::JsBuiltin;
use crate::js_hotkey::JsHotKey;
use crate::js_keycode::build_keycode;
use crate::js_operation::JsOperation;
use crate::plugin::Plugins;

pub struct JS {
    context: Context,
    js_operation_rx: Option<Receiver<JsOperation>>,
    monitoring_queue: Option<Arc<RwLock<VecDeque<Event>>>>,
    plugins: Option<Plugins>,
    plugin_id2filename: HashMap<String, String>,
}

impl JS {
    pub fn new(
        js_operation_rx: Option<Receiver<JsOperation>>,
        monitoring_queue: Option<Arc<RwLock<VecDeque<Event>>>>,
        plugins: Option<Plugins>,
    ) -> anyhow::Result<Self> {
        let context = Context::default();

        let mut js = JS {
            context,
            js_operation_rx,
            monitoring_queue,
            plugins,
            plugin_id2filename: HashMap::new(),
        };
        js.init_console()?;
        js.init_hotkey()?;
        js.init_keycode()?;
        js.register_constants()?;
        js.register_builtin_functions()?;
        js.load_driver()?;
        js.load_bundled()?;
        Ok(js)
    }

    pub fn get_filename_by_plugin_id(&self, plugin_id: &String) -> Option<String> {
        let result = self.plugin_id2filename.get(plugin_id).cloned();
        if result == None {
            log::info!(
                "unknown plugin_id({})... current filename map is {:?}",
                plugin_id,
                self.plugin_id2filename
            );
        }
        result
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
        self.register_constant(js_string!("Key"), keycode)?;
        Ok(())
    }

    fn register_constants(&mut self) -> anyhow::Result<()> {
        self.register_constant(js_string!("kCGEventKeyDown"), CGEventType_kCGEventKeyDown)?;
        self.register_constant(js_string!("kCGEventKeyUp"), CGEventType_kCGEventKeyUp)?;
        self.register_constant(js_string!("kCGEventFlagsChanged"), CGEventType_kCGEventFlagsChanged)?;
        self.register_constant(
            js_string!("kCGKeyboardEventKeycode"),
            CGEventField_kCGKeyboardEventKeycode,
        )?;
        self.register_constant(
            js_string!("kCGEventFlagMaskNonCoalesced"),
            CGEventFlags_kCGEventFlagMaskNonCoalesced,
        )?;

        // CGEventFlags
        self.register_constant(
            js_string!("kCGEventFlagMaskAlphaShift"),
            CGEventFlags_kCGEventFlagMaskAlphaShift,
        )?;
        self.register_constant(js_string!("kCGEventFlagMaskShift"), CGEventFlags_kCGEventFlagMaskShift)?;
        self.register_constant(
            js_string!("kCGEventFlagMaskControl"),
            CGEventFlags_kCGEventFlagMaskControl,
        )?;
        self.register_constant(
            js_string!("kCGEventFlagMaskAlternate"),
            CGEventFlags_kCGEventFlagMaskAlternate,
        )?;
        self.register_constant(
            js_string!("kCGEventFlagMaskCommand"),
            CGEventFlags_kCGEventFlagMaskCommand,
        )?;
        self.register_constant(js_string!("kCGEventFlagMaskHelp"), CGEventFlags_kCGEventFlagMaskHelp)?;
        self.register_constant(
            js_string!("kCGEventFlagMaskSecondaryFn"),
            CGEventFlags_kCGEventFlagMaskSecondaryFn,
        )?;
        self.register_constant(
            js_string!("kCGEventFlagMaskNumericPad"),
            CGEventFlags_kCGEventFlagMaskNumericPad,
        )?;

        Ok(())
    }

    fn register_constant<K, V>(&mut self, key: K, value: V) -> anyhow::Result<()>
        where
            K: Into<PropertyKey> + Debug,
            V: Into<JsValue>,
    {
        if let Err(err) = self
            .context
            .register_global_property(key, value, Attribute::READONLY)
        {
            return Err(anyhow!("Cannot register constant: {:?}", err));
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
                context.register_global_callable(js_string!(name), 1, NativeFunction::from_fn_ptr(fn_ptr))
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
            "$$loadAppConfigJson",
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
            .get(js_string!("$$invokeEvent"), &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$invokeEvent: {:?}", err))?;
        let invoke_event = JsFunction::try_from_js(&invoke_event, &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$invokeEvent as JsFunction: {:?}", err))?;

        let mut needs_config_reload = false;
        let operations = self.get_js_operations();
        for op in operations {
            match op {
                JsOperation::ReloadConfig => {
                    needs_config_reload = true;
                }
                JsOperation::ReloadPlugins => {
                    if let Err(err) = self.reload_plugins() {
                        log::error!("cannot reload plugin: {:?}", err)
                    }
                }
                JsOperation::UnloadPlugin { plugin_id, .. } => {
                    if let Err(err) = self.unload_plugin(plugin_id.clone()) {
                        log::error!("cannot unload plugin({}): {:?}", plugin_id, err)
                    }
                }
            }
        }

        let event = Event::from_cf(cg_event_type, cg_event_ref);
        if let Some(queue) = &self.monitoring_queue {
            match queue.write() {
                Ok(mut queue) => {
                    queue.push_back(event.clone());
                    if queue.len() > 40 {
                        queue.pop_front();
                    }
                }
                Err(err) => {
                    log::error!("Cannot get lock for monitoring: {:?}", err)
                }
            }
        }

        let js_key_event = self.build_key_event(&event, cg_event_type)?;
        let result = invoke_event
            .call(
                &JsValue::undefined(),
                &[
                    JsValue::from(js_key_event),
                    JsValue::Boolean(needs_config_reload),
                ],
                &mut self.context,
            )
            .map_err(|err| anyhow!("Cannot call $$invokeEvent as JsFunction: {:?}", err))?;
        let result = result.as_boolean().unwrap_or(true);
        Ok(result)
    }

    fn get_js_operations(&mut self) -> Vec<JsOperation> {
        let mut result = Vec::new();
        if let Some(rx) = &self.js_operation_rx {
            'out: loop {
                match rx.try_recv() {
                    Ok(op) => {
                        log::info!("Received operation: {:?}", op);
                        result.push(op);
                    }
                    Err(err) => match err {
                        TryRecvError::Empty => {
                            log::debug!("needs_plugin_reload: empty");
                            break 'out;
                        }
                        TryRecvError::Disconnected => {
                            log::warn!("needs_plugin_reload: disconnected");
                            break 'out;
                        }
                    },
                }
            }
        }
        result
    }

    fn unload_plugin(&mut self, plugin_id: String) -> anyhow::Result<()> {
        log::info!("Trying to unload plugin: {}", plugin_id);

        let unload_plugin = self
            .context
            .global_object()
            .get(js_string!("$$unloadPlugin"), &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$unloadPlugin: {:?}", err))?;
        let unload_plugin = JsFunction::try_from_js(&unload_plugin, &mut self.context)
            .map_err(|err| anyhow!("Cannot get $$unloadPlugin as JsFunction: {:?}", err))?;
        unload_plugin
            .call(
                &JsValue::undefined(),
                &[JsValue::from(js_string!(plugin_id))],
                &mut self.context,
            )
            .map_err(|err| anyhow!("Cannot call $$unloadPlugin as JsFunction: {:?}", err))?;
        Ok(())
    }

    fn build_key_event(
        &mut self,
        event: &Event,
        cg_event_type: CGEventType,
    ) -> anyhow::Result<JsObject> {
        let key_event = JsObject::with_object_proto(self.context.intrinsics());

        fn set<K, V>(js: &mut JS, key_event: &JsObject, key: K, value: V) -> anyhow::Result<()>
            where
                K: Into<PropertyKey>,
                V: Into<JsValue>,
        {
            if let Err(err) = key_event.set(key, value, false, &mut js.context) {
                return Err(anyhow!("Cannot set name: {:?}", err));
            }
            Ok(())
        }

        set(
            self,
            &key_event,
            js_string!("type"),
            js_string!(event.event_type.as_str()),
        )?;

        set(
            self,
            &key_event,
            js_string!("keycode"),
            JsValue::from(event.keycode),
        )?;

        if cg_event_type == CGEventType_kCGEventFlagsChanged {
            set(
                self,
                &key_event,
                js_string!("flags"),
                JsValue::from(event.flags),
            )?;
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

    fn reload_plugins(&mut self) -> anyhow::Result<()> {
        log::info!("JS::reload_plugins");
        self.load_user_scripts()
    }

    pub fn load_user_scripts(&mut self) -> anyhow::Result<()> {
        log::info!("Trying to load plugins");

        if let Some(plugins) = &self.plugins {
            let plugin_snippets = plugins.read_user_scripts()?;

            let mut last_loaded_plugins = self.loaded_plugins()?;

            for plugin_snippet in plugin_snippets {
                let filename = plugin_snippet.filename.as_str();

                if let Err(err) = self.eval(plugin_snippet.src) {
                    log::error!("Cannot load {}: {:?}", plugin_snippet.filename, err)
                }

                let current_loaded_plugins = self.loaded_plugins()?;

                let diff = current_loaded_plugins.difference(&last_loaded_plugins);
                for plugin_id in diff {
                    self.plugin_id2filename
                        .insert(plugin_id.to_string(), filename.to_string());
                }

                last_loaded_plugins = current_loaded_plugins;
            }
        }
        Ok(())
    }

    pub fn loaded_plugins(&mut self) -> anyhow::Result<HashSet<String>> {
        let config_schema_list = self.get_config_schema()?;
        let result: HashSet<String> = config_schema_list
            .plugins
            .iter()
            .map(|f| f.id.to_string())
            .collect();
        Ok(result)
    }

    pub fn get_config_schema(&mut self) -> anyhow::Result<ConfigSchemaList> {
        let get_config_schema = self
            .context
            .global_object()
            .get(js_string!("$$getConfigSchema"), &mut self.context)
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
pub struct ConfigSchemaList {
    pub plugins: Vec<ConfigSchema>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSchema {
    pub id: String,
    name: String,
    description: String,
    config: Vec<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_schema() -> anyhow::Result<()> {
        let mut js = JS::new(None, None, None)?;
        let schema = js.get_config_schema()?;
        assert_eq!(schema.plugins.first().unwrap().id, "builtin.dynamicmacro");
        assert_eq!(schema.plugins.first().unwrap().name, "Dynamic Macro");
        log::info!("schema={:?}", schema);
        Ok(())
    }

    #[test]
    fn test_eval() -> anyhow::Result<()> {
        let mut js = JS::new(None, None, None)?;
        let value = js.eval("3+4".to_string())?;
        let got = value.to_u32(&mut js.context).unwrap();
        assert_eq!(got, 7);
        Ok(())
    }
}
