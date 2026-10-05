#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sum() {
        assert_eq!(sum(&[3, 1, 4, 1, 5, 9, 2, 6]), 31);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn check_max() {
        assert_eq!(max(&[-3, -1, -4]), Some(&-1));
        assert_eq!(max(&[]), None);
    }

    #[test]
    fn check_min() {
        assert_eq!(min(&[3, 1, 4]), Some(&1));
        assert_eq!(min(&[]), None);
    }

    #[test]
    fn check_evens() {
        assert_eq!(evens(&[3, 0, -2, 4, 4]), vec![0, -2, 4, 4]);
        assert!(evens(&[]).is_empty());
    }

    #[test]
    fn check_squares() {
        assert_eq!(squares(&[-2, 0, 3]), vec![4, 0, 9]);
        assert!(squares(&[]).is_empty());
    }

    #[test]
    fn check_count_over_four() {
        assert_eq!(count_over_four(&[4, 5, 9, 2, 6]), 3);
        assert_eq!(count_over_four(&[]), 0);
    }
}
