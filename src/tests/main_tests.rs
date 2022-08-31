#[cfg(test)]
mod main_tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn zero_liters_at_zero_distance() {
        // * Arrange
        let mock_distance = 0 as u32;
        let mock_production_year = 1991 as u16;
        let mock_fuel_usage_per_100km = 4 as u16;

        // ! Act
        let result = super::super::calculate_dissel_usage_for_distance(mock_distance, mock_production_year, mock_fuel_usage_per_100km);

        // ? Assert
        assert_eq!(result, "0");
    }

    #[test]
    fn zero_liters_at_zero_fuel_consumption() {
        // * Arrange
        let mock_distance = 725 as u32;
        let mock_production_year = 1991 as u16;
        let mock_fuel_usage_per_100km = 0 as u16;

        // ! Act
        let result = super::super::calculate_dissel_usage_for_distance(mock_distance, mock_production_year, mock_fuel_usage_per_100km);

        // ? Assert
        assert_eq!(result, "0");
    }

}