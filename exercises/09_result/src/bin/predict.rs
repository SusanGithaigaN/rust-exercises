// TOPIC 09 — Result<T, E>
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, write down the output of each println.
//
// Questions to answer first:
//   1. What does the ? operator do inside double_str?
//   2. What is the return type of double_str?
//   3. What does .map() do to an Err value?
//   4. What does .unwrap_or(0) do when Result is Err?
//
// Run: cargo run --bin predict

use std::num::ParseIntError;

fn double_str(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", double_str("5"));
    println!("{:?}", double_str("abc"));
    println!("{:?}", double_str("-3"));

    let result = double_str("10").unwrap_or(0);
    println!("{}", result);

    let failed = double_str("nope").unwrap_or(0);
    println!("{}", failed);

    // What does this chain produce?
    let chained = double_str("3")
        .map(|n| n + 1)
        .map(|n| n.to_string());
    println!("{:?}", chained);

    let chained_err = double_str("x")
        .map(|n| n + 1);
    println!("{:?}", chained_err);
}
