use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys::Array;

use crate::{bindings, call_method::call_method, model::Model};

pub fn get_adjacent_pairs(model: Model) -> Vec<(u32, u32)> {
    let var_name: &Vec<JsValue> = &vec![model.into()];
    let result = call_method(
        &bindings::UTIL,
        &"getAdjacentPairs".into(),
        &Array::from_iter(var_name.iter()),
    )
    .unwrap();
    serde_wasm_bindgen::from_value(result).unwrap()
}
