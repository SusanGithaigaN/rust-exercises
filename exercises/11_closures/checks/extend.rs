#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_apply_twice() {
        assert_eq!(apply_twice(|x| x + 10, 3), 23);
        assert_eq!(apply_twice(|x| x * 2, 3), 12);
    }

    #[test]
    fn check_apply_to_each() {
        let v = vec![1, 2, 3];
        assert_eq!(apply_to_each(&v, |x| x * x), vec![1, 4, 9]);
        assert_eq!(v, vec![1, 2, 3]);
        assert!(apply_to_each(&[], |x| x + 1).is_empty());
    }

    #[test]
    fn check_compose() {
        let f = compose(|x| x + 1, |x| x * 2);
        assert_eq!(f(3), 7);
        assert_eq!(f(0), 1);
    }
}
