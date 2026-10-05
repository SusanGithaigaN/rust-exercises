#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sorted_copy() {
        let v = [3.0, 1.0, 2.0];
        assert_eq!(sorted_copy(&v), vec![1.0, 2.0, 3.0]);
        assert_eq!(v, [3.0, 1.0, 2.0]);
        assert!(sorted_copy(&[]).is_empty());
    }

    #[test]
    fn check_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn check_median() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn check_mode() {
        assert_eq!(mode(&[1, 2, 2, 3, 3, 3]), Some(3));
        assert_eq!(mode(&[2, 1, 2, 1]), Some(1));
        assert_eq!(mode(&[]), None);
    }
}
