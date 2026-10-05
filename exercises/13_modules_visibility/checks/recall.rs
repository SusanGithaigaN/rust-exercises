#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_point() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert_eq!((b.x, b.y), (3.0, 4.0));
        assert!((a.distance(&b) - 5.0).abs() < 1e-9);
        assert_eq!(a.distance(&a), 0.0);
    }

    #[test]
    fn check_circle() {
        let c = Circle::new(Point::new(0.0, 0.0), 10.0);
        assert!((c.area() - std::f64::consts::PI * 100.0).abs() < 1e-9);
        assert!(c.contains(&Point::new(3.0, 4.0)));
        assert!(c.contains(&Point::new(10.0, 0.0)));
        assert!(!c.contains(&Point::new(10.0, 10.0)));
    }
}
