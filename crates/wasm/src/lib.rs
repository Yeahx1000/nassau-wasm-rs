use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct MetricsDTO {
    pub point_of_sail: String,
    pub optimal_sail_angle: f64,
    pub trim_score: u8,
    pub speed_factor: f64,
    pub notes: String,
}

#[wasm_bindgen]
pub fn compute_metrics (tws: f64, twa: f64, sail_angle: f64) -> JsValue {
    let metrics = nassau_core::compute_metrics(tws, twa, sail_angle);
    
    let dto = MetricsDTO {
        point_of_sail: metrics.point_of_sail.as_str().to_string(),
        optimal_sail_angle: metrics.optimal_sail_angle,
        trim_score: metrics.trim_score,
        speed_factor: metrics.speed_factor,
        notes: metrics.notes,
    };

    serde_wasm_bindgen::to_value(&dto).unwrap_or(JsValue::NULL)

}