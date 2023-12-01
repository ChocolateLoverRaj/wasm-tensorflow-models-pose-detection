use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{
    js_sys::{Array, Promise},
    JsFuture,
};

use crate::{call_method::call_method, pose::Pose};

#[derive(Clone)]
pub struct PoseDetector {
    js_value: JsValue,
}

impl From<JsValue> for PoseDetector {
    fn from(value: JsValue) -> Self {
        PoseDetector { js_value: value }
    }
}

impl PoseDetector {
    pub async fn estimate_poses(
        &self,
        image: JsValue,
        timestamp: i32,
    ) -> Result<Vec<Pose>, JsValue> {
        let poses = JsFuture::from(Promise::from(call_method(
            &self.js_value,
            &"estimatePoses".into(),
            &Array::from_iter(vec![image, timestamp.into()].iter()),
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

impl Drop for PoseDetector {
    fn drop(&mut self) {
        self.dispose();
    }
}
