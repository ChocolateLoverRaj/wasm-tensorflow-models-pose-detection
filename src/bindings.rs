use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Promise;

#[wasm_bindgen(module = "@tensorflow-models/pose-detection")]
extern "C" {
    #[wasm_bindgen(js_name = createDetector)]
    pub fn create_detector(model: &str, model_config: &JsValue) -> Promise;

    #[wasm_bindgen(js_name = util)]
    pub static UTIL: JsValue;
}
