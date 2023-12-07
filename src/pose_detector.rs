use crate::{call_method::call_method, pose::Pose};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{
    js_sys::{Array, Object, Promise},
    JsFuture,
};

#[derive(Clone)]
pub struct PoseDetector {
    js_value: JsValue,
}

impl JsCast for PoseDetector {
    fn instanceof(val: &JsValue) -> bool {
        // I'm pretty sure there is no `PoseDetector` class in JavaScript.
        Object::instanceof(val)
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        PoseDetector { js_value: val }
    }

    fn unchecked_from_js_ref(_val: &JsValue) -> &Self {
        panic!("unchecked_from_js_ref not implemented for PoseDetector");
    }
}

impl AsRef<JsValue> for PoseDetector {
    fn as_ref(&self) -> &JsValue {
        &self.js_value
    }
}

impl From<JsValue> for PoseDetector {
    fn from(value: JsValue) -> Self {
        PoseDetector { js_value: value }
    }
}

impl Into<JsValue> for PoseDetector {
    fn into(self) -> JsValue {
        self.js_value
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonEstimationConfig {
    pub max_poses: Option<u32>,
    pub flip_horizontal: Option<bool>,
}
impl Into<JsValue> for CommonEstimationConfig {
    fn into(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PoseNetEstimationConfig {
    #[serde(skip_serializing)]
    pub common_config: CommonEstimationConfig,
    pub score_threshold: Option<f64>,
    pub nms_radius: Option<f64>,
}
impl Into<JsValue> for PoseNetEstimationConfig {
    fn into(self) -> JsValue {
        let common_config: Object = serde_wasm_bindgen::to_value(&self.common_config)
            .unwrap()
            .dyn_into()
            .unwrap();
        let pose_net_config: Object = serde_wasm_bindgen::to_value(&self)
            .unwrap()
            .dyn_into()
            .unwrap();
        Object::assign2(&Object::new(), &common_config, &pose_net_config).into()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EstimationConfig {
    PoseNet(PoseNetEstimationConfig),
    BlazePoseOrMoveNet(CommonEstimationConfig),
}
impl Into<JsValue> for EstimationConfig {
    fn into(self) -> JsValue {
        match self {
            EstimationConfig::PoseNet(config) => config.into(),
            EstimationConfig::BlazePoseOrMoveNet(config) => config.into(),
        }
    }
}

impl PoseDetector {
    pub async fn estimate_poses(
        &self,
        image: &JsValue,
        config: EstimationConfig,
        timestamp: Option<i32>,
    ) -> Result<Vec<Pose>, JsValue> {
        let inputs = Array::from_iter(vec![image, &config.into(), &timestamp.into()].iter());
        let poses = JsFuture::from(Promise::from(call_method(
            &self.js_value,
            &"estimatePoses".into(),
            &inputs,
        )?))
        .await?;
        let poses = Array::from(&poses)
            .to_vec()
            .into_iter()
            .map(|pose| Pose::try_from(pose).unwrap())
            .collect::<Vec<_>>();
        Ok(poses)
    }

    /// The `Drop` trait is implemented to call this function, so in Rust you can just drop it instead of calling this function directly.
    pub fn dispose(&self) {
        call_method(&self.js_value, &"dispose".into(), &Array::new()).unwrap();
    }

    pub fn reset(&self) {
        call_method(&self.js_value, &"reset".into(), &Array::new()).unwrap();
    }
}
