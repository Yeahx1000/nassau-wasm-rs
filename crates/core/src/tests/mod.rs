#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_optimal_sail_angle() {
        let a = optimal_sail_angle(0.0);
        let b = optimal_sail_angle(90.0);
        let c = optimal_sail_angle(180.0);
        assert!(a < b && b < c);
    }

    #[test]
    fn trim_score_bounds(){
        let metric = compute_metrics(12.0, 90.0, 45.0);
        assert!(metric.trim_score <= 100);
    }

    #[test]
    fn point_of_sail_classification(){
        assert_eq!(classify_point_of_sail(10.0).as_str(), "Close Hauled");
        assert_eq!(classify_point_of_sail(90.0).as_str(), "Beam Reach");
        assert_eq!(classify_point_of_sail(175.0).as_str(), "Running");


    }

}