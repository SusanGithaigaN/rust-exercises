// TOPIC 11 — Closures
// EXERCISE TYPE: Extend
// =============================================
// apply_twice and apply_to_each from recall are provided. Add one more.
//
// ADD:
// fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
// where
//     F: Fn(i32) -> i32,
//     G: Fn(i32) -> i32,
//
// Returns a new closure that applies g first, then f.
// compose(f, g)(x) == f(g(x))
//
// In main:
//   - Create closures: add1 = |x| x + 1, double = |x| x * 2
//   - Use compose to make add1_then_double (double first, add1 second — read carefully)
//   - Apply to vec [1, 2, 3, 4, 5] using apply_to_each
//
// Expected output (add1 after double):
//   [3, 5, 7, 9, 11]
//
// Run: cargo run --bin extend

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    v.iter().map(|&x| f(x)).collect()
}

// TODO: add compose here

fn main() {
    let add_ten = |x: i32| x + 10;
    println!("apply_twice add_ten to 3: {}", apply_twice(add_ten, 3));
    println!("squares: {:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x * x));
    // TODO: test compose here
}
