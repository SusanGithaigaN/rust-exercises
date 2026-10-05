// TOPIC 04 — Borrowing: Mutable References
// EXERCISE TYPE: Recall
// =============================================
// Write three functions that mutate values through mutable references.
// The caller keeps ownership — the function only borrows.
//
// 1. fn append_greeting(s: &mut String)
//    Appends ", hello!" to s. No return value needed.
//
// 2. fn double_all(v: &mut Vec<i32>)
//    Doubles every element of v in place.
//    Hint: for x in v.iter_mut() { *x *= 2; }
//
// 3. fn clamp_to_zero(n: &mut i32)
//    If *n is negative, set it to 0.
//
// In main:
//   - Create a String, call append_greeting, print result
//   - Create a Vec<i32>, call double_all, print result
//   - Create an i32, call clamp_to_zero on both a negative and positive value
//
// Expected output:
//   greeting: world, hello!
//   doubled: [2, 6, 10, 14]
//   clamped -5: 0
//   clamped 3: 3
//
// Run: cargo run --bin recall

fn main() {
    // TODO: implement the three functions above, then use them here
}
