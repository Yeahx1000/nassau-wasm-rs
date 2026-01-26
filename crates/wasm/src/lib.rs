use wasm_bindgen::prelude::*;
us serde::Serialize;

#[derive(Serialize)]
pub struct Metrics {
    pub point_of_sail: String,
    pub optimal_sail_angle: f64,
    pub trim_score: u8,
    pub speed_factor: f64,
    pub notes: String,
}

#[wasm_bindgen]
pub fn compute_metrics (tws: f64, twa: f64, sail_angle: f64) -> JsValue {
    
}