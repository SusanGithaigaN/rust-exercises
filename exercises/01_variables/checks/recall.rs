#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_double() {
        assert_eq!(double(5), 10);
        assert_eq!(double(0), 0);
        assert_eq!(double(-3), -6);
    }

    #[test]
    fn check_describe_sign() {
        assert_eq!(describe_sign(7), "positive");
        assert_eq!(describe_sign(0), "zero");
        assert_eq!(describe_sign(-4), "negative");
    }

    #[test]
    fn check_celsius_to_fahrenheit() {
        assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-9);
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-9);
        assert!((celsius_to_fahrenheit(-40.0) + 40.0).abs() < 1e-9);
    }
}
