// TOPIC 10 — Vec and Iteration
// EXERCISE TYPE: Recall
// =============================================
// Use iterator methods only — no manual index loops (no for i in 0..v.len()).
//
// Given: let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
//
// Compute and print the following using iterator chains:
//   1. The sum of all elements
//   2. The maximum value
//   3. The minimum value
//   4. All even numbers collected into a new Vec<i32>
//   5. The square of each number collected into a new Vec<i32>
//   6. The count of numbers greater than 4
//
// Hints:
//   .sum::<i32>()
//   .max() / .min()  <- returns Option
//   .filter(|&&x| ...)
//   .map(|&x| ...)
//   .count()
//
// Expected output:
//   sum: 31
//   max: 9
//   min: 1
//   evens: [4, 2, 6]
//   squares: [9, 1, 16, 1, 25, 81, 4, 36]
//   count > 4: 3
//
// Run: cargo run --bin recall

fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // TODO: compute and print each result above using iterator methods
}
