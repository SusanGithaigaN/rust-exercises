#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
        assert_eq!(first_even(&[-3, -2, 0]), Some(-2));
        assert_eq!(first_even(&[1, 3]), None);
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn check_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
        assert_eq!(safe_divide(0.0, 2.0), Some(0.0));
        assert_eq!(safe_divide(10.0, 0.0), None);
    }

    #[test]
    fn check_head() {
        assert_eq!(head(&[9, 2, 3]), Some(9));
        assert_eq!(head(&[0]), Some(0));
        assert_eq!(head(&[]), None);
    }

    #[test]
    fn check_find_index() {
        assert_eq!(find_index(&[5, 3, 5], 5), Some(0));
        assert_eq!(find_index(&[1, 3, 5], 5), Some(2));
        assert_eq!(find_index(&[1, 3], 5), None);
        assert_eq!(find_index(&[], 5), None);
    }

    #[test]
    fn check_last_element() {
        assert_eq!(last_element(&[9, 2, 3]), Some(3));
        assert_eq!(last_element(&[0]), Some(0));
        assert_eq!(last_element(&[]), None);
    }
}
