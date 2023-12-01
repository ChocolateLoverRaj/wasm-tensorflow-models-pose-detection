use model::Model;
use pose_detector::PoseDetector;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console::{log_1, log_2};

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
    let name = model.get_name();
    let detector_js_value = JsFuture::from(bindings::create_detector(&model.get_name()[..], &{
        let c = model.get_config();
        log_1(&"create_detector".into());
        log_2(&name[..].into(), &c);
        c
    }))
    .await?;
    Ok(PoseDetector::from(detector_js_value))
}
