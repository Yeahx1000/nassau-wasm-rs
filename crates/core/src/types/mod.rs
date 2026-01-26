pub enum PointOfSail {
    CloseHauled,
    CloseReach,
    BeamReach,
    BroadReach,
    Running,
}

impl PointOfSail {
    pub fn as_str(&self) -> &'static str {
        match self {
            PointOfSail::CloseHauled => "Close Hauled",
            PointOfSail::CloseReach => "Close Reach",
            PointOfSail::BeamReach => "Beam Reach",
            PointOfSail::BroadReach => "Broad Reach",
            PointOfSail::Running => "Running",
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