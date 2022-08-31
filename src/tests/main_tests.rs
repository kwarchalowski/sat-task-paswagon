#[cfg(test)]
mod main_tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    #[test]
    fn probability_has_to_be_in_0_100_range() {
        assert!(mycargo::myutils::random_percentage() > 0);
        assert!(mycargo::myutils::random_percentage() < 100);
    }

    #[test]
    fn zero_liters_at_zero_distance() {
        // Arrange
        let mock_distance = 0 as u32;
        let mock_production_year = 1991 as u16;
        let mock_fuel_usage_per_100km = 4.20 as f32;

        // Act
        let mock_result = super::super::calculate_dissel_usage_for_distance(mock_distance, mock_production_year, mock_fuel_usage_per_100km);

        // Assert
        assert_eq!(mock_result, "0");
    }

}