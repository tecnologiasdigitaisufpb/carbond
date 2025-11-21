pub mod carbon_intensity;
pub mod cpu_cycles;
pub mod metric;

/// Rounds a number to 4 decimal places.
pub(self) fn round(number: f64) -> f64 {
    (number * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_decimal_places() {
        let x = 12.34567_f64;
        let expected = 12.3457_f64;

        let actual = round(x);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_2_decimal_places() {
        let x = 12.32_f64;
        let expected = 12.32_f64;

        let actual = round(x);

        assert_eq!(expected, actual);
    }
}
