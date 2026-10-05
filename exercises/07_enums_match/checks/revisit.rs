#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_area() {
        assert!((area(&Shape::Circle(3.0)) - std::f64::consts::PI * 9.0).abs() < 1e-9);
        assert_eq!(area(&Shape::Rectangle(4.0, 6.0)), 24.0);
        assert!((area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-9);
    }

    #[test]
    fn check_describe() {
        assert_eq!(describe(&Shape::Circle(3.0)), "Circle with radius 3");
        assert_eq!(describe(&Shape::Rectangle(4.0, 6.0)), "Rectangle 4 x 6");
        assert_eq!(
            describe(&Shape::Triangle(3.0, 4.0, 5.0)),
            "Triangle with sides 3, 4, 5"
        );
    }

    #[test]
    fn check_is_valid() {
        assert!(is_valid(&Shape::Circle(3.0)));
        assert!(!is_valid(&Shape::Circle(0.0)));
        assert!(!is_valid(&Shape::Rectangle(-1.0, 2.0)));
        assert!(is_valid(&Shape::Triangle(3.0, 4.0, 5.0)));
        assert!(!is_valid(&Shape::Triangle(1.0, 2.0, 3.0)));
        assert!(!is_valid(&Shape::Triangle(1.0, 2.0, 10.0)));
        assert!(!is_valid(&Shape::Circle(f64::INFINITY)));
    }
}
