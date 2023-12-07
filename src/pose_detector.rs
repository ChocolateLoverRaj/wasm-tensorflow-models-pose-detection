use crate::{call_method::call_method, pose::Pose};
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

impl PoseDetector {
    pub async fn estimate_poses(
        &self,
        image: &JsValue,
        timestamp: Option<i32>,
    ) -> Result<Vec<Pose>, JsValue> {
        let inputs = Array::from_iter(vec![image, &JsValue::UNDEFINED, &timestamp.into()].iter());
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
