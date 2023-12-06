use model::Model;
use pose_detector::PoseDetector;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

mod bindings;
mod call_method;
pub mod model;
pub mod pose;
pub mod pose_detector;

pub enum BackendName {
    Webgl,
    Cpu,
    Tensorflow,
}

impl ToString for BackendName {
    fn to_string(&self) -> String {
        match self {
            BackendName::Cpu => "cpu",
            BackendName::Webgl => "webgl",
            BackendName::Tensorflow => "tensorflow",
        }
        .into()
    }
}

pub async fn create_detector(model: Model) -> Result<PoseDetector, JsValue> {
    let detector_js_value = JsFuture::from(bindings::create_detector(
        &model.get_name()[..],
        &model.get_config(),
    ))
    .await?;
    Ok(PoseDetector::from(detector_js_value))
}
