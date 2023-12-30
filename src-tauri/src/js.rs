use std::fmt::Debug;
use anyhow::anyhow;
use apple_sys::CoreGraphics::{CGEventField_kCGKeyboardEventKeycode, CGEventType_kCGEventFlagsChanged, CGEventType_kCGEventKeyDown, CGEventType_kCGEventKeyUp};
use boa_engine::{Context, js_string, JsValue, NativeFunction, Source};
use boa_engine::object::builtins::{JsArray, JsMap};
use boa_engine::property::{Attribute, PropertyKey};
use boa_gc::{Finalize, GcRefCell, Trace};
use boa_runtime::Console;

#[derive(Debug, Clone, Trace, Finalize)]
struct BigStruct {
    id_list: JsArray,
    callbacks: JsMap,
}

pub struct JS<'a> {
    context: Context<'a>,
    big_struct: BigStruct,
}
impl JS<'_> {
    pub fn new() -> anyhow::Result<Self> {
        let mut context = Context::default();

        let big_struct = BigStruct {
            id_list: JsArray::new(&mut context),
            callbacks: JsMap::new(&mut context),
        };
        let mut js = JS {
            context,
            big_struct,
        };
        js.init_console()?;
        js.register_constants()?;
        js.register_register_plugin()?;
        return Ok(js);
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

    fn register_constants(&mut self) -> anyhow::Result<()> {
        self.register_constant("kCGEventKeyDown", CGEventType_kCGEventKeyDown)?;
        self.register_constant("kCGEventKeyUp", CGEventType_kCGEventKeyUp)?;
        self.register_constant("kCGEventFlagsChanged", CGEventType_kCGEventFlagsChanged)?;
        self.register_constant("kCGKeyboardEventKeycode", CGEventField_kCGKeyboardEventKeycode)?;
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

    fn register_register_plugin(&mut self) -> anyhow::Result<()> {
        unsafe {
            // this, args, etc.

            if let Err(err) = self.context.register_global_callable("register_plugin", 1, NativeFunction::from_closure_with_captures(
                move |_this, args, captures, context| {
                    log::info!("print11!!!! {:?}", args);

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

                    Ok(JsValue::from(js_string!("hello")))
                },
                GcRefCell::new(self.big_struct.clone())
            )) {
                return Err(anyhow!("Cannot register `register_plugin` function: {:?}", err));
            }
        }

        Ok(())
    }

    pub fn eval(&mut self, src: &String) -> anyhow::Result<JsValue> {
        return match self.context.eval(Source::from_bytes(src)) {
            Ok(value) => { Ok(value) }
            Err(err) => {
                Err(anyhow!("Cannot execute javascript code: {:?}", err))
            }
        }
    }

    // Call this method when key/mouse event was received.
    // This method calls JS handlers.
    pub fn send_event(&mut self) -> anyhow::Result<()> {
        let len = match self.big_struct.id_list.length(&mut self.context) {
            Ok(len) => { len }
            Err(err) => {
                return Err(anyhow!("Cannot get the length of ids: {:?}", err))
            }
        };
        for i in 0..len {
            let id = match self.big_struct.id_list.get(i, &mut self.context) {
                Ok(id) => { id }
                Err(err) => {
                    return Err(anyhow!("Cannot get id: {:?}", err))
                }
            };
            log::debug!("Calling {:?}", id);
            let callback = match self.big_struct.callbacks.get(id.clone(), &mut self.context) {
                Ok(callback) => { callback }
                Err(err) => {
                    return Err(anyhow!("Cannot get callback: {:?}, {:?}", err, id))
                }
            };
            log::debug!("Callback={:?}", callback);
            let got = match callback.as_callable().unwrap()
                .call(&JsValue::Null, &[JsValue::from("HOGEHOGE")], &mut self.context) {
                Ok(got) => {got}
                Err(err) => {
                    return Err(anyhow!("Cannot call the handler: {:?}", err));
                }
            };
            if got.to_boolean() {
                log::debug!("Do not propagate...")
            } else {
                log::debug!("propagate the event")
            }
        }
        Ok(())
    }
}
