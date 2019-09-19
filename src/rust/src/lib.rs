#[macro_use]
extern crate serde_derive;
extern crate js_sys;

use rand::prelude::*;

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn to_hex(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    name: String,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    #[serde(rename = "ruleSamples")]
    rule_samples: u64,
    #[serde(rename = "paintSamples")]
    paint_samples: u64,
    cardinality: u8,
    #[serde(rename = "centerSampleWidth")]
    center_sample_width: f64,
    #[serde(rename = "outerSampleWidth")]
    outer_sample_width: f64,
    #[serde(rename = "sampleSpread")]
    sample_spread: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    states: HashMap<String, State>,
    settings: Settings,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rule {}

mod generate_rules {
    use super::*;

    fn get_image_data_for_area(
        training_canvas_context: &CanvasRenderingContext2d,
        cx: f64,
        cy: f64,
        r: f64,
    ) -> Result<ImageData, JsValue> {
        training_canvas_context.get_image_data(cx - r, cy - r, cx + r, cy + r)
    }

    fn get_color_distance_in_circle(image_data: &ImageData, color: &Color) -> f64 {
        let width = image_data.width() as i32;
        let height = image_data.height() as i32;
        let data = image_data.data();

        let mut distance = 0.0;

        for (i, chunk) in data.chunks(4).enumerate() {
            let x = i as i32 % width;
            let y = i as i32 / width;
            let dx = x - width / 2;
            let dy = y - height / 2;

            if (dx.pow(2) + dy.pow(2) > (width / 2).pow(2)) {
                continue;
            }

            let r = chunk[0] as i32;
            let g = chunk[1] as i32;
            let b = chunk[2] as i32;
            let a = chunk[3] as i32;

            distance += ((r - color.r as i32) as i32).abs() as f64;
            distance += ((g - color.g as i32) as i32).abs() as f64;
            distance += ((b - color.b as i32) as i32).abs() as f64;
        }

        distance
    }

    fn get_most_prominent_state_in_circle<'a>(
        training_canvas_context: &CanvasRenderingContext2d,
        cx: f64,
        cy: f64,
        r: f64,
        states: &'a HashMap<String, State>,
        output_to_canvas: &CanvasRenderingContext2d,
    ) -> Result<&'a State, JsValue> {
        let image_data = get_image_data_for_area(training_canvas_context, cx, cy, r)?;

        let state = states
            .values()
            .min_by_key(|state| get_color_distance_in_circle(&image_data, &state.color) as u64)
            .expect("bar");

        output_to_canvas.set_fill_style(&JsValue::from_str(state.color.to_hex().as_str()));
        output_to_canvas.begin_path();
        output_to_canvas.arc(cx, cy, r, 0.0, PI * 2.0).expect("2");
        output_to_canvas.fill();

        Ok(state)
    }

    fn generate_rule(
        training_canvas_context: &CanvasRenderingContext2d,
        output_to_canvas: &CanvasRenderingContext2d,
        config: &Config,
        rng: &mut SmallRng,
    ) -> Result<Rule, JsValue> {
        let x = rng.gen_range(
            0.0,
            256.0,
            //config.settings.sample_spread + config.settings.outer_sample_width,
            //256.0 - (config.settings.sample_spread + config.settings.outer_sample_width),
        );
        let y = rng.gen_range(0.0, 256.0);

        get_most_prominent_state_in_circle(
            training_canvas_context,
            x,
            y,
            config.settings.center_sample_width,
            &config.states,
            output_to_canvas,
        );

        Ok(Rule {})
    }

    #[wasm_bindgen]
    pub fn generate_rules(
        config_js: &JsValue,
        training_canvas_context: &CanvasRenderingContext2d,
        sample_canvas_context: &CanvasRenderingContext2d,
    ) -> Result<JsValue, JsValue> {
        let config: Config = config_js.into_serde().map_err(|e| {
            JsValue::from_str(
                format!("[{:?}] ({:?}) could not parse input command", config_js, e).as_str(),
            )
        })?;

        let epoch_milis = js_sys::Date::now();

        let mut rng = SmallRng::seed_from_u64(epoch_milis as u64);

        let mut rules: Vec<Rule> = vec![];

        for i in 0..config.settings.rule_samples {
            let rule = generate_rule(
                training_canvas_context,
                sample_canvas_context,
                &config,
                &mut rng,
            )?;
            rules.push(rule);
        }

        Ok(JsValue::from_serde(&rules).map_err(|_| "Error stringifying output")?)

        //Err(JsValue::from_str("foo"))
    }

}

pub use generate_rules::generate_rules;

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
