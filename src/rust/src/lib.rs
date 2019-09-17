#[macro_use]
extern crate serde_derive;
extern crate js_sys;

use rand::prelude::*;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    name: String,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    #[serde(default = "default_u8")]
    cardinality: u8,
    #[serde(default = "default_f64", rename = "centerSampleWidth")]
    center_sample_width: f64,
    #[serde(default = "default_f64", rename = "outerSampleWidth")]
    outer_sample_width: f64,
    #[serde(default = "default_f64", rename = "sampleSpread")]
    sample_spread: f64,
}

fn default_u8() -> u8 {
    3
}

fn default_f64() -> f64 {
    32.0
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    states: HashMap<String, State>,
    settings: Settings,
}

#[wasm_bindgen]
pub fn render_settings_preview(
    config_js: &JsValue,
    canvas_context: &CanvasRenderingContext2d,
) -> Result<JsValue, JsValue> {
    let config: Config = config_js.into_serde().map_err(|e| {
        JsValue::from_str(
            format!("[{:?}] ({:?}) could not parse input command", config_js, e).as_str(),
        )
    })?;

    canvas_context.clear_rect(0.0, 0.0, 256.0, 256.0);

    canvas_context.begin_path();
    canvas_context
        .arc(
            128.0,
            128.0,
            config.settings.center_sample_width,
            0.0,
            PI * 2.0,
        )
        .unwrap();
    canvas_context.stroke();

    for i in 0..config.settings.cardinality {
        let theta: f64 = (i as f64 / config.settings.cardinality as f64 - 0.25) * PI * 2.0;

        canvas_context.begin_path();
        canvas_context
            .arc(
                128.0 + config.settings.sample_spread * theta.cos(),
                128.0 + config.settings.sample_spread * theta.sin(),
                config.settings.outer_sample_width,
                0.0,
                PI * 2.0,
            )
            .unwrap();
        canvas_context.stroke();
    }

    Ok(JsValue::from_serde(&config).map_err(|_| "Error stringifying output")?)
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    Ok(())
}
