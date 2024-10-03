pub fn percentage_to_int(value: f64) -> i32 {
    (value * 100).round() as i32;
}

pub fn fraction_to_int(numerator: i32, denominator: i32) -> i32 {
    (numerator as f64 / denominator as f64 * 100.0).round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_to_int() {
        assert_eq!(percentage_to_int(0.123), 12);
        assert_eq!(percentage_to_int(0.456), 46);
        assert_eq!(percentage_to_int(0.789), 79);
    }

    #[test]
    fn test_fraction_to_int() {
        assert_eq!(fraction_to_int(1, 2), 50);
        assert_eq!(fraction_to_int(2, 3), 67);
        assert_eq!(fraction_to_int(3, 4), 75);
    }
}
