#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_append_exclamation() {
        assert_eq!(append_exclamation(String::from("hello")), "hello!");
        assert_eq!(append_exclamation(String::new()), "!");
    }

    #[test]
    fn check_shout() {
        assert_eq!(shout(String::from("hello!")), "HELLO!");
        assert_eq!(shout(String::new()), "");
    }

    #[test]
    fn check_double_elements() {
        assert_eq!(double_elements(vec![1, -2, 0]), vec![2, -4, 0]);
        assert_eq!(double_elements(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn check_sum_and_drop() {
        assert_eq!(sum_and_drop(vec![1, 2, 3]), 6);
        assert_eq!(sum_and_drop(vec![-2, 2]), 0);
        assert_eq!(sum_and_drop(vec![]), 0);
    }
}
