// TOPIC 11 — Closures
// EXERCISE TYPE: Recall
// =============================================
// Write closures and higher-order functions from scratch.
//
// 1. Create a closure called `add_ten` that takes an i32 and returns i32 + 10.
//
// 2. Write a function:
//      fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32
//    Applies f to x, then applies f again to the result.
//
// 3. Write a function:
//      fn apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32>
//    Returns a new Vec where f has been applied to every element.
//
// In main:
//   - Use add_ten with apply_twice (should give x + 20)
//   - Use apply_to_each with a closure that squares its argument
//   - Use apply_to_each with a closure that captures an external variable `n`
//     and adds it to each element
//
// Expected output (with n = 5):
//   apply_twice(add_ten, 3) = 23
//   squares of [1,2,3,4,5]: [1, 4, 9, 16, 25]
//   add 5 to [1,2,3,4,5]: [6, 7, 8, 9, 10]
//
// Run: cargo run --bin recall

fn main() {
    // TODO: define add_ten closure, implement apply_twice and apply_to_each,
    // then use them here to produce the expected output
}
