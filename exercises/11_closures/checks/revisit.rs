#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_make_adder() {
        let add = make_adder(5);
        assert_eq!(add(10), 15);
        assert_eq!(add(-5), 0);
    }

    #[test]
    fn check_make_multiplier() {
        let mul = make_multiplier(3);
        assert_eq!(mul(4), 12);
        assert_eq!(mul(0), 0);
    }

    #[test]
    fn check_pipeline() {
        let add = |x| x + 5;
        let mul = |x| x * 3;
        assert_eq!(pipeline(&[1, 2], &[&add, &mul]), vec![18, 21]);
        assert_eq!(pipeline(&[1, 2], &[]), vec![1, 2]);
        assert!(pipeline(&[], &[&add]).is_empty());
    }
}
