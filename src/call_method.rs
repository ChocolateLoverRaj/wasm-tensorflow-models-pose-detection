use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::js_sys::{Array, Reflect};

pub fn call_method(target: &JsValue, key: &JsValue, inputs: &Array) -> Result<JsValue, JsValue> {
    let function_js_value = Reflect::get(target, key)?;
    let function = match function_js_value.dyn_ref() {
        Some(v) => v,
        None => return Err(JsValue::undefined()),
    };
    let output = Reflect::apply(function, target, inputs)?;
    Ok(output)
}
