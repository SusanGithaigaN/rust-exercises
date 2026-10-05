// TOPIC 08 — Option<T>
// EXERCISE TYPE: Recall
// =============================================
// Write three functions that return Option<T> instead of panicking.
//
// 1. fn first_even(v: &[i32]) -> Option<i32>
//    Returns the first even number in the slice, or None.
//
// 2. fn safe_divide(a: f64, b: f64) -> Option<f64>
//    Returns None if b is 0.0, otherwise Some(a / b).
//
// 3. fn head(v: &[i32]) -> Option<i32>
//    Returns the first element, or None if the slice is empty.
//
// In main: call each function with inputs that produce Some AND None.
// Handle the results with match (not .unwrap()).
//
// Expected output:
//   first even in [1,3,4,7]: Some(4)
//   first even in [1,3,5,7]: None
//   10 / 2 = Some(5.0)
//   10 / 0 = None
//   head of [9,2,3]: Some(9)
//   head of []: None
//
// Run: cargo run --bin recall

fn main() {
    // TODO: implement the three functions, then use them here
}
