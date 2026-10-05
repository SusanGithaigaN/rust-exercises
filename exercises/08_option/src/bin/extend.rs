// TOPIC 08 — Option<T>
// EXERCISE TYPE: Extend
// =============================================
// The three recall functions are provided. Add two more and chain them.
//
// ADD:
// 1. fn find_index(v: &[i32], target: i32) -> Option<usize>
//    Returns the index of the first occurrence of target, or None.
//    Hint: v.iter().position(|&x| x == target)
//
// 2. fn last_element(v: &[i32]) -> Option<i32>
//    Returns a copy of the last element, or None if empty.
//    Hint: v.last().copied()
//
// Also demonstrate chaining with .and_then():
// In main, write code that:
//   - Finds the index of a value
//   - If found, returns the element at that index squared (use and_then)
//   - Prints the result
//
// Expected output:
//   index of 5 in [1,3,5,7,9]: Some(2)
//   index of 4 in [1,3,5,7,9]: None
//   last of [9,2,3]: Some(3)
//   last of []: None
//   index then square: Some(25)
//
// Run: cargo run --bin extend

fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn head(v: &[i32]) -> Option<i32> {
    v.first().copied()
}

// TODO: add find_index and last_element here

fn main() {
    let v = vec![1, 3, 5, 7, 9];
    println!("first even in v: {:?}", first_even(&v));
    println!("head: {:?}", head(&v));
    // TODO: test find_index, last_element, and the and_then chain here
}
