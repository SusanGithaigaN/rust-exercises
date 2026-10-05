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

    #[test]
    fn check_swap() {
        assert_eq!(swap(3, 7), (7, 3));
        assert_eq!(swap(-1, 5), (5, -1));
        assert_eq!(swap(2, 2), (2, 2));
    }

    #[test]
    fn check_is_between() {
        assert!(is_between(5, 1, 10));
        assert!(is_between(1, 1, 10));
        assert!(is_between(10, 1, 10));
        assert!(!is_between(0, 1, 10));
        assert!(!is_between(11, 1, 10));
    }
}
