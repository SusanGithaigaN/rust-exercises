#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_remove_duplicates() {
        let mut v = vec![3, 1, 3, 2, 1];
        remove_duplicates(&mut v);
        assert_eq!(v, vec![3, 1, 2]);
        let mut v = vec![];
        remove_duplicates(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn check_normalize() {
        let mut v = vec![1.0, 2.0, 3.0];
        normalize(&mut v);
        assert_eq!(v.len(), 3);
        assert!((v[0] - 1.0 / 3.0).abs() < 1e-9);
        assert!((v[1] - 2.0 / 3.0).abs() < 1e-9);
        assert!((v[2] - 1.0).abs() < 1e-9);
        let mut v = vec![0.0, 0.0];
        normalize(&mut v);
        assert_eq!(v, vec![0.0, 0.0]);
        let mut v = vec![];
        normalize(&mut v);
        assert!(v.is_empty());
    }
}
