// TOPIC 11 — Closures
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK A: fn make_adder(n: i32) -> impl Fn(i32) -> i32
//   Returns a closure that adds n to its argument.
//   Requires `move` to capture n.
//
// TASK B: fn make_multiplier(n: i32) -> impl Fn(i32) -> i32
//   Returns a closure that multiplies its argument by n.
//
// TASK C: fn pipeline(v: &[i32], ops: &[&dyn Fn(i32) -> i32]) -> Vec<i32>
//   Applies each operation in ops to each element.
//   Each op is applied once per element, in order.
//   Hint: for each element, fold over ops applying each in turn.
//
// In main:
//   - Create add5 and mul3
//   - Apply pipeline to [1,2,3,4,5] with [add5, mul3] — add 5 first, then multiply by 3
//
// Expected output:
//   make_adder(5)(10) = 15
//   make_multiplier(3)(4) = 12
//   pipeline [1,2,3,4,5] with [+5, *3]: [18, 21, 24, 27, 30]
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement make_adder, make_multiplier, and pipeline above main, then use here
}
