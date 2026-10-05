// TOPIC 14 — Writing Tests
// EXERCISE 01: Test simple functions
// =============================================
// Three functions are implemented below.
// YOUR TASK: write the #[cfg(test)] module with tests for all three.
//
// Requirements:
//   - At least 3 tests per function (9 total minimum)
//   - Cover: normal cases, edge cases, and boundary values
//   - Use assert_eq! and assert! appropriately
//   - Give every test a descriptive name
//
// Think before writing each test:
//   - What is the simplest valid input?
//   - What happens at the boundaries (0, negative, max value)?
//   - What should happen with unusual but valid input?
//
// Run: cargo test --bin exercise_01

fn double(n: i32) -> i32 {
    n * 2
}

fn clamp(n: i32, min: i32, max: i32) -> i32 {
    if n < min { min } else if n > max { max } else { n }
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// TODO: write your tests here
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // your tests go here
// }

fn main() {
    println!("double(5) = {}", double(5));
    println!("clamp(15, 0, 10) = {}", clamp(15, 0, 10));
    println!("reverse \"hello\" = {}", reverse_string("hello"));
    println!("Run 'cargo test --bin exercise_01' to run your tests.");
}
