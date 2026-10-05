#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new() {
        let r = Rectangle::new(4.0, 6.0);
        assert_eq!(r.width, 4.0);
        assert_eq!(r.height, 6.0);
    }

    #[test]
    fn check_area() {
        let r = Rectangle {
            width: 4.0,
            height: 6.0,
        };
        assert_eq!(r.area(), 24.0);
        assert_eq!(r.width, 4.0);
        assert_eq!(
            Rectangle {
                width: 0.0,
                height: 6.0
            }
            .area(),
            0.0
        );
    }

    #[test]
    fn check_perimeter() {
        assert_eq!(
            Rectangle {
                width: 4.0,
                height: 6.0
            }
            .perimeter(),
            20.0
        );
    }

    #[test]
    fn check_is_square() {
        assert!(Rectangle {
            width: 5.0,
            height: 5.0
        }
        .is_square());
        assert!(!Rectangle {
            width: 4.0,
            height: 6.0
        }
        .is_square());
    }

    #[test]
    fn check_scale() {
        let r = Rectangle {
            width: 4.0,
            height: 6.0,
        };
        let bigger = r.scale(2.0);
        assert_eq!((bigger.width, bigger.height), (8.0, 12.0));
        assert_eq!((r.width, r.height), (4.0, 6.0));
        let zero = r.scale(0.0);
        assert_eq!((zero.width, zero.height), (0.0, 0.0));
    }

    #[test]
    fn check_can_contain() {
        let big = Rectangle {
            width: 10.0,
            height: 8.0,
        };
        let small = Rectangle {
            width: 5.0,
            height: 3.0,
        };
        assert!(big.can_contain(&small));
        assert!(!small.can_contain(&big));
        assert!(!big.can_contain(&big));
    }

    #[test]
    fn check_circle() {
        let c = Circle::new(3.0);
        assert_eq!(c.radius, 3.0);
        assert!((c.area() - std::f64::consts::PI * 9.0).abs() < 1e-9);
        assert!((c.circumference() - std::f64::consts::PI * 6.0).abs() < 1e-9);
        assert_eq!(Circle::new(0.0).area(), 0.0);
    }
}
