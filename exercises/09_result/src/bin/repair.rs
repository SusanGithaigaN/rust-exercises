// Read ../../README.md before starting.

use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

// ERROR 1
fn print_doubled(s: &str) {
    let n = s.parse::<i32>()?;
    println!("doubled: {}", n * 2);
}

fn main() {
    // ERROR 2 — runtime panic
    let result = parse_and_double("not_a_number").unwrap();
    println!("{}", result);

    // ERROR 3
    parse_and_double("5");

    // ERROR 4
    let val = parse_and_double("abc").unwrap_or(|| 0);
    println!("{}", val);

    print_doubled("7");
}
