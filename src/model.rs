use core::panic;
use std::fmt::Display;

use serde::{Serialize, Serializer};
use strum_macros::Display;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys::{Object, Reflect};

#[derive(Display)]
pub enum PoseNetArchitecture {
    ResNet50,
    MobileNetV1,
}

#[repr(i32)]
pub enum PoseNetOutputStride {
    Is32 = 32,
    Is16 = 16,
    Is8 = 8,
}

pub enum MobileNetMultiplier {
    Is1,
    Is0Point5,
    Is0Point75,
}
impl Into<f64> for MobileNetMultiplier {
    fn into(self) -> f64 {
        match self {
            MobileNetMultiplier::Is1 => 1.0,
            MobileNetMultiplier::Is0Point5 => 0.5,
            MobileNetMultiplier::Is0Point75 => 0.75,
        }
    }
}

#[repr(i32)]
pub enum QuantBytes {
    Is1 = 1,
    Is2 = 2,
    Is4 = 4,
}

pub struct InputResolution {
    pub width: i32,
    pub height: i32,
}

pub struct PoseNetModelConfig {
    pub architecture: PoseNetArchitecture,
    pub output_stride: PoseNetOutputStride,
    pub input_resolution: InputResolution,
    pub multiplier: Option<MobileNetMultiplier>,
    pub model_url: Option<String>,
    pub quant_bytes: Option<QuantBytes>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlazePoseMediaPipeModelConfig {
    pub solution_path: Option<String>,
}
impl Into<JsValue> for BlazePoseMediaPipeModelConfig {
    fn into(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlazePoseTfjsModelConfig {
    // TODO: Also allow io.IOHandler
    pub detector_model_url: Option<String>,
    // TODO: Also allow io.IOHandler
    pub landmark_model_url: Option<String>,
}
impl Into<JsValue> for BlazePoseTfjsModelConfig {
    fn into(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

pub enum Runtime {
    Mediapipe(BlazePoseMediaPipeModelConfig),
    Tfjs(BlazePoseTfjsModelConfig),
}
impl Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            Self::Mediapipe(_) => "mediapipe",
            Self::Tfjs(_) => "tfjs",
        };
        write!(f, "{string}")
    }
}
impl Into<JsValue> for Runtime {
    fn into(self) -> JsValue {
        let runtime = self.to_string();
        let o: JsValue = match self {
            Self::Mediapipe(c) => c.into(),
            Self::Tfjs(c) => c.into(),
        };
        Reflect::set(&o, &"runtime".into(), &runtime.into()).unwrap();
        o
    }
}

pub enum BlazePoseModelType {
    Lite,
    Full,
    Heavy,
}
impl Display for BlazePoseModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            Self::Lite => "lite",
            Self::Full => "full",
            Self::Heavy => "heavy",
        };
        write!(f, "{string}")
    }
}
impl Serialize for BlazePoseModelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string()[..])
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlazePoseModelConfig {
    #[serde(skip_serializing)]
    pub runtime: Runtime,
    pub enable_smoothing: Option<bool>,
    pub enable_segmentation: Option<bool>,
    pub smooth_segmentation: Option<bool>,
    pub model_type: Option<BlazePoseModelType>,
}

impl Into<JsValue> for BlazePoseModelConfig {
    fn into(self) -> JsValue {
        let config = Object::from(serde_wasm_bindgen::to_value(&self).unwrap());
        Object::assign(
            &config,
            &Object::from({
                let runtime_config: JsValue = self.runtime.into();
                runtime_config
            }),
        );
        config.into()
    }
}

pub enum TrackerType {
    Keypoint,
    BoundingBox,
}
impl Display for TrackerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            Self::Keypoint => "keypoint",
            Self::BoundingBox => "boundingBox",
        };
        write!(f, "{string}")
    }
}

pub struct KeypointTrackerConfig {
    pub keypoint_confidence_threshold: i32,
    pub keypoint_falloff: Vec<i32>,
    pub min_number_of_keypoints: i32,
}

pub struct BoundingBoxTrackerConfig;

pub struct TrackerConfig {
    pub max_tracks: i32,
    pub max_age: i32,
    pub min_similarity: i32,
    pub keypoint_tracker_params: Option<KeypointTrackerConfig>,
    pub bounding_box_tracker_params: Option<BoundingBoxTrackerConfig>,
}

pub struct MoveNetModelConfig {
    pub enable_smoothing: Option<bool>,
    pub model_type: Option<String>,
    // TODO: Also allow io.IOHandler
    pub model_url: Option<String>,
    pub min_pose_score: Option<i32>,
    pub multi_pose_max_dimension: Option<i32>,
    pub enable_tracking: Option<bool>,
    pub tracker_type: Option<TrackerType>,
    pub tracker_config: Option<TrackerConfig>,
}

pub enum Model {
    PoseNet(Option<PoseNetModelConfig>),
    BlazePose(Option<BlazePoseModelConfig>),
    MoveNet(Option<MoveNetModelConfig>),
}

impl Model {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::PoseNet(_) => "PoseNet",
            Self::BlazePose(_) => "BlazePose",
            Self::MoveNet(_) => "MoveNet",
        }
    }

    pub fn get_config(self) -> JsValue {
        match self {
            Self::BlazePose(blaze_pose_model_config) => match blaze_pose_model_config {
                Some(config) => config.into(),
                None => JsValue::undefined(),
            },
            _ => panic!("Not implemented. Make an issue :)"),
        }
    }
}
