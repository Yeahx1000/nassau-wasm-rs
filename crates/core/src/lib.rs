mod tests;
mod types;
use types::{Metrics, PointOfSail};



fn clamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

/*
Inputs: 
- tws: true wind speed: in know (0..=60 ish)
- twa: true wind angle, compared to bow in degrees (0..=180)
- sail_angle: sail angle compared to centerline in degrees (0..=90)
*/ 

pub fn compute_metrics (tws: f64, twa: f64, sail_angle: f64) -> Metrics {
    let twa_clamped = clamp(twa, 0.0, 180.0);
    let tws_clamped = clamp(tws, 0.0, 60.0);
    let sail_angle_clamped: f64 = clamp(sail_angle, 0.0, 90.0); 

    let pos: PointOfSail = classify_point_of_sail(twa_clamped);
    let optimal: f64 = optimal_sail_angle(twa_clamped);

    let error = (sail_angle_clamped - optimal).abs();

    // Trim Scoring - lose 3 points per degree off 
    let mut score = 100.0 - (3.0 * error);

    score = clamp(score, 0.0, 100.0);
    let trim_score = score.round() as u8;

    // Base speed curve vs twa (0..180) with peek near 90 (beam reach)
    // triangle curve
    // - 0 deg => 0.25
    // - 90 deg => 1.0
    // - 180 deg => 0.55
    let base = base_speed_factor(twa_clamped);

    let wind_bonus = clamp(tws_clamped / 20.0, 0.0, 1.0);

    let trim_multiplier = 0.5 + (trim_score as f64 / 200.0);
    let speed_factor = clamp(base * (0.8 + 0.2 * wind_bonus) * trim_multiplier, 0.0, 1.0);

    let notes = trim_hint(sail_angle_clamped, optimal);

    Metrics {
        point_of_sail: pos.as_str().to_string(),
        optimal_sail_angle: optimal,
        trim_score,
        speed_factor,
        notes,
    }
}

pub fn classify_point_of_sail (twa: f64) -> PointOfSail {
    match twa {
        x if x < 45.0 => PointOfSail::CloseHauled,
        x if x < 70.0 => PointOfSail::CloseReach,
        x if x < 110.0 => PointOfSail::BeamReach,
        x if x < 150.0 => PointOfSail::BroadReach,
        _ => PointOfSail::Running,
    }
}

pub fn optimal_sail_angle (twa: f64) -> f64 {
    let twa = clamp(twa, 0.0, 180.0);

    let points = [
        (0.0, 12.0),
        (45.0, 18.0),
        (90.0 , 45.0),
        (135.0, 70.0),
        (180.0, 85.0),
    ];



    for w in points.windows(2) {
        let (x0, y0) = w[0];
        let (x1, y1) = w[1];

        if twa >= x0 && twa <= x1 {
            let t = if x1 == x0 {0.0} else {(twa - x0) / (x1 - x0)};
            return y0 + t * (y1 - y0);
        }
    }

    85.0
}

fn base_speed_factor(twa: f64) -> f64 {
    let twa = clamp(twa, 0.0, 180.0);

    if twa <= 90.0 {
        let t = twa / 90.0;
        0.2 + t * (1.0 - 0.2)
    } else {
        let t = (twa - 90.0) / 90.0;
        1.0 - t * (0.55 - 1.0)
    }
}

fn trim_hint(actual:f64, optimal: f64) -> String {
    let diff = actual - optimal;

    if diff.abs() < 3.0 {
        "Trim is optimal.".to_string()
    } else if diff > 0.0 {
        format!("Trim in ~{} degrees.", diff.abs().round() as i32)
    } else {
        format!("Ease ~{} degrees.", diff.abs().round() as i32)
    }
}

