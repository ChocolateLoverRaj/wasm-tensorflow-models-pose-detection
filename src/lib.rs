use model::ModelWithConfig;
use pose_detector::PoseDetector;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

mod bindings;
mod call_method;
pub mod model;
pub mod pose;
pub mod pose_detector;
pub mod util;

pub async fn create_detector(model: ModelWithConfig) -> Result<PoseDetector, JsValue> {
    let name = &model.get_name()[..];
    let config = model.get_config();
    let detector_js_value = JsFuture::from(bindings::create_detector(name, &config))
        .await
        .unwrap();
    Ok(PoseDetector::from(detector_js_value))
}
