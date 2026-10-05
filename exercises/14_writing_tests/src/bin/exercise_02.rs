// TOPIC 14 — Writing Tests
// EXERCISE 02: Test Option and Result functions
// =============================================
// Two functions are implemented below — one returns Option, one returns Result.
// YOUR TASK: write tests for both.
//
// For Option functions, test:
//   - Cases that return Some (what is the value?)
//   - Cases that return None (under what conditions?)
//
// For Result functions, test:
//   - Cases that return Ok (what is the value?)
//   - Cases that return Err (which error? under what input?)
//
// Useful patterns:
//   assert_eq!(find_even(&[1, 3, 4]), Some(4));
//   assert_eq!(find_even(&[1, 3, 5]), None);
//   assert!(parse_positive("5").is_ok());
//   assert!(parse_positive("abc").is_err());
//   assert_eq!(parse_positive("5"), Ok(5));
//
// Run: cargo test --bin exercise_02

fn find_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

fn parse_positive(s: &str) -> Result<u32, String> {
    if s.is_empty() {
        return Err(String::from("input is empty"));
    }
    match s.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{}' is not a valid positive number", s)),
    }
}

// TODO: write your tests here
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // your tests go here
// }

fn main() {
    println!("find_even([1,3,4]) = {:?}", find_even(&[1, 3, 4]));
    println!("parse_positive(\"5\") = {:?}", parse_positive("5"));
    println!("Run 'cargo test --bin exercise_02' to run your tests.");
}
