pub enum Point_Of_Sail {
    Close_Hauled,
    Close_Reach,
    Beam_Reach,
    Broad_Reach,
    Running,
}

impl Point_Of_Sail {
    pub fn as_str($self) -> &'static str {
        match self {
            Point_Of_Sail::Close_Hauled => "Close Hauled",
            Point_Of_Sail::Close_Reach => "Close Reach",
            Point_Of_Sail::Beam_Reach => "Beam Reach",
            Point_Of_Sail::Broad_Reach => "Broad Reach",
            Point_Of_Sail::Running => "Running",
        }
    }
}

pub struct Metrics {
    pub point_of_sail: String,
    pub optimal_sail_angle: f64,
    pub trim_score: u8,
    pub speed_factor: f64,
    pub notes: String,
}

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
    let sail_angle_clamped = clamp(sail_angle, 0.0, 90.0); 
}