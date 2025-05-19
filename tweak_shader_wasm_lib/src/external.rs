use serde::{Deserialize, Serialize};
use tweak_shader::input_type::InputType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JsInputValue {
    Float {
        current: f32,
        min: f32,
        max: f32,
        default: f32,
    },
    Int {
        current: i32,
        min: i32,
        max: i32,
        default: i32,
        labels: Option<Vec<(String, i32)>>,
    },
    Point {
        current: [f32; 2],
        min: [f32; 2],
        max: [f32; 2],
        default: [f32; 2],
    },
    Bool {
        current: bool,
        default: bool,
    },
    Color {
        current: [f32; 4],
        default: [f32; 4],
    },
    Image {
        status: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    RawBytes {
        bytes: Vec<u8>,
    },
}

impl From<&InputType> for JsInputValue {
    fn from(input: &InputType) -> Self {
        match input {
            InputType::Float(bounded) => JsInputValue::Float {
                current: bounded.current,
                min: bounded.min,
                max: bounded.max,
                default: bounded.default,
            },
            InputType::Int(bounded, labels) => JsInputValue::Int {
                current: bounded.current,
                min: bounded.min,
                max: bounded.max,
                default: bounded.default,
                labels: labels.clone(),
            },
            InputType::Point(bounded) => JsInputValue::Point {
                current: bounded.current,
                min: bounded.min,
                max: bounded.max,
                default: bounded.default,
            },
            InputType::Bool(discrete) => JsInputValue::Bool {
                current: discrete.current.is_true(),
                default: discrete.default.is_true(),
            },
            InputType::Color(discrete) => JsInputValue::Color {
                current: discrete.current,
                default: discrete.default,
            },
            InputType::Image(status) => match status {
                tweak_shader::input_type::TextureStatus::Uninit => JsInputValue::Image {
                    status: "uninit".to_string(),
                    width: None,
                    height: None,
                },
                tweak_shader::input_type::TextureStatus::Loaded { width, height } => {
                    JsInputValue::Image {
                        status: "loaded".to_string(),
                        width: Some(*width),
                        height: Some(*height),
                    }
                }
            },
            InputType::RawBytes(raw) => JsInputValue::RawBytes {
                bytes: raw.inner.clone(),
            },
        }
    }
}
