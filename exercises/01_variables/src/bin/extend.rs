// TOPIC 01 — Variables, Mutability, Functions
// EXERCISE TYPE: Extend
// =============================================
// The three functions from recall are provided below.
// Add two NEW functions without modifying the existing ones.
//
// ADD:
// 1. fn swap(a: i32, b: i32) -> (i32, i32)
//    Returns the two values in reversed order as a tuple.
//
// 2. fn is_between(value: i32, low: i32, high: i32) -> bool
//    Returns true if low <= value <= high (both ends inclusive).
//
// Expected output:
//   swap(3, 7) = (7, 3)
//   swap(-1, 5) = (5, -1)
//   5 between 1 and 10: true
//   11 between 1 and 10: false
//   1 between 1 and 10: true
//   0 between 1 and 10: false
//
// Run: cargo run --bin extend

fn double(n: i32) -> i32 {
    n * 2
}

fn describe_sign(n: i32) -> &'static str {
    if n > 0 { "positive" } else if n < 0 { "negative" } else { "zero" }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// TODO: add swap and is_between here

fn main() {
    println!("double(5) = {}", double(5));
    println!("sign of -3: {}", describe_sign(-3));
    println!("100°C = {}°F", celsius_to_fahrenheit(100.0));
    // TODO: call and print swap and is_between with the expected values above
}
