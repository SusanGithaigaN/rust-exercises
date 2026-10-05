// TOPIC 09 — Result<T, E>
// EXERCISE TYPE: Repair
// =============================================
// Fix the Result-handling errors.
// Some will panic at runtime — a note marks those.
//
// Run: cargo run --bin repair

use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

// ERROR 1: ? is used in a function that does not return Result
fn print_doubled(s: &str) {
    let n = s.parse::<i32>()?;
    println!("doubled: {}", n * 2);
}

fn main() {
    // ERROR 2 (runtime panic): .unwrap() on a known error value
    let result = parse_and_double("not_a_number").unwrap();
    println!("{}", result);

    // ERROR 3: the Result is silently ignored (Rust warns about this)
    parse_and_double("5");

    // ERROR 4: wrong method — trying to use .unwrap_or() with a closure argument
    // unwrap_or takes a value, unwrap_or_else takes a closure — pick the right one
    let val = parse_and_double("abc").unwrap_or(|| 0);
    println!("{}", val);

    print_doubled("7");
}
