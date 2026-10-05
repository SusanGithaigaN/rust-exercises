#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_append_greeting() {
        let mut s = "world".into();
        append_greeting(&mut s);
        assert_eq!(s, "world, hello!");
        let mut s = String::new();
        append_greeting(&mut s);
        assert_eq!(s, ", hello!");
    }

    #[test]
    fn check_double_all() {
        let mut v = vec![1, -2, 0];
        double_all(&mut v);
        assert_eq!(v, vec![2, -4, 0]);
        let mut v = vec![];
        double_all(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn check_clamp_to_zero() {
        let mut n = -5;
        clamp_to_zero(&mut n);
        assert_eq!(n, 0);
        let mut n = 3;
        clamp_to_zero(&mut n);
        assert_eq!(n, 3);
        let mut n = 0;
        clamp_to_zero(&mut n);
        assert_eq!(n, 0);
    }
}
