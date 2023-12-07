use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Keypoint {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub score: Option<f64>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pose {
    pub keypoints: Vec<Keypoint>,
    pub score: Option<f64>,
    pub keypoints_3d: Option<Vec<Keypoint>>,
    pub bounding_box: Option<BoundingBox>,
    // TODO: segmentation
    pub id: Option<f64>,
}

impl TryFrom<JsValue> for Pose {
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(serde_wasm_bindgen::from_value::<Self>(value)?)
    }

    type Error = serde_wasm_bindgen::Error;
}
