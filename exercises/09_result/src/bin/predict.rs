// Read ../../README.md before starting.

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

    let chained = double_str("3").map(|n| n + 1).map(|n| n.to_string());
    println!("{:?}", chained);

    let chained_err = double_str("x").map(|n| n + 1);
    println!("{:?}", chained_err);
}
