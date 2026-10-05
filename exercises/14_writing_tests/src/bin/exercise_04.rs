// TOPIC 14 — Writing Tests
// EXERCISE 04: Test panicking functions
// =============================================
// Two functions are implemented below — one panics intentionally.
// YOUR TASK: write tests including at least one #[should_panic] test.
//
// #[should_panic] marks a test that MUST panic to pass:
//
//   #[test]
//   #[should_panic]
//   fn test_panics_on_empty() {
//       get_element(&[], 0); // this should panic
//   }
//
// You can also check the panic message:
//
//   #[test]
//   #[should_panic(expected = "index out of bounds")]
//   fn test_panics_with_message() {
//       get_element(&[1, 2, 3], 10);
//   }
//
// YOUR REQUIREMENTS:
//   - At least 3 passing tests for get_element (non-panicking cases)
//   - At least 2 #[should_panic] tests for get_element
//   - At least 2 tests for safe_divide
//
// Run: cargo test --bin exercise_04

fn get_element(v: &[i32], index: usize) -> i32 {
    if index >= v.len() {
        panic!("index {} out of bounds for length {}", index, v.len());
    }
    v[index]
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

// TODO: write your tests here
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // your tests go here
// }

fn main() {
    let v = vec![10, 20, 30];
    println!("element at 1: {}", get_element(&v, 1));
    println!("10 / 2: {:?}", safe_divide(10, 2));
    println!("10 / 0: {:?}", safe_divide(10, 0));
    println!("Run 'cargo test --bin exercise_04' to run your tests.");
}
