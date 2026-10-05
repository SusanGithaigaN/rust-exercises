// Read ../../README.md before starting.

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_two_positives() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_multiply_by_zero() {
        assert_eq!(multiply(5, 0), 0);
    }

    #[test]
    fn test_multiply_negatives() {
        assert_eq!(multiply(-2, -3), 6);
    }

    #[test]
    fn test_is_even_with_even_number() {
        assert!(is_even(4));
    }

    #[test]
    fn test_is_even_with_odd_number() {
        assert!(!is_even(7));
    }

    #[test]
    fn test_is_even_with_zero() {
        assert!(is_even(0));
    }
}

fn main() {
    println!("Run 'cargo test --bin read_first' to run the tests.");
    println!("Then move on to exercise_01 through exercise_04.");
}
