// TOPIC 14 — Writing Tests
// READ THIS FIRST
// =============================================
// This topic is different from the others.
// Here YOU write the tests. No test code is provided.
//
// --- HOW TESTS WORK IN RUST ---
//
// A test module lives in the same file as the code it tests:
//
//   fn add(a: i32, b: i32) -> i32 {
//       a + b
//   }
//
//   #[cfg(test)]
//   mod tests {
//       use super::*;   // import everything from the parent module
//
//       #[test]
//       fn test_add_positive_numbers() {
//           assert_eq!(add(2, 3), 5);
//       }
//
//       #[test]
//       fn test_add_negative() {
//           assert_eq!(add(-1, -2), -3);
//       }
//
//       #[test]
//       fn test_add_zero() {
//           assert_eq!(add(0, 5), 5);
//       }
//   }
//
// --- RUNNING TESTS ---
//
//   cargo test                          <- runs all tests in the project
//   cargo test --bin read_first         <- runs tests in this file only
//   cargo test test_add                 <- runs tests whose name contains "test_add"
//
// --- USEFUL MACROS ---
//
//   assert!(expr)              <- panics if expr is false
//   assert_eq!(left, right)    <- panics if left != right, shows both values
//   assert_ne!(left, right)    <- panics if left == right
//
//   #[should_panic]            <- marks a test that is expected to panic
//   #[should_panic(expected = "some message")]  <- checks the panic message too
//
// --- NAMING TESTS ---
//
// Good test names describe what is being tested and what outcome is expected:
//   test_divide_by_zero_returns_none     <- good
//   test1                                <- useless name
//
// --- WHAT TO TEST ---
//
// For each function, think about:
//   1. The normal/happy path (typical input -> expected output)
//   2. Edge cases (empty input, zero, negative numbers, boundary values)
//   3. Error cases (invalid input -> returns None or Err or panics)
//
// --- YOUR TASK FOR THIS FILE ---
//
// Study the example below until you understand every line.
// Then run: cargo test --bin read_first
// You should see one passing test.
//
// Run: cargo test --bin read_first

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
