#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sum_to() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(5), 15);
        assert_eq!(sum_to(100), 5050);
    }
}
