use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct Keypoint {
    pub x: i32,
    pub y: i32,
    pub z: Option<i32>,
    pub score: Option<i32>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BoundingBox {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Pose {
    pub keypoints: Vec<Keypoint>,
    pub score: Option<i32>,
    pub keypoints_3d: Option<Vec<Keypoint>>,
    pub bounding_box: Option<BoundingBox>,
    // TODO: segmentation
    pub id: Option<i32>,
}

impl TryFrom<JsValue> for Pose {
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(serde_wasm_bindgen::from_value::<Self>(value)?)
    }

    type Error = serde_wasm_bindgen::Error;
}
