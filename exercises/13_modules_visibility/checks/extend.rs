#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_modules() {
        let p = Point::new(0.0, 0.0);
        assert_eq!(p.distance(&Point::new(3.0, 4.0)), 5.0);
        let c = Circle::new(p, 10.0);
        assert!((c.area() - std::f64::consts::PI * 100.0).abs() < 1e-9);
        assert!(c.contains(&Point::new(10.0, 0.0)));
        assert!(!c.contains(&Point::new(10.0, 10.0)));
    }
}
