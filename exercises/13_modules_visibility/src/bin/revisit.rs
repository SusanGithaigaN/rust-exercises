// TOPIC 13 — Modules and Visibility
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Build a stats module with a private implementation and a clean public API.
//
// mod stats {
//     // Private helper — not visible outside the module:
//     fn sorted_copy(v: &[f64]) -> Vec<f64>  <- returns sorted copy
//
//     // Public API:
//     pub fn mean(v: &[f64]) -> Option<f64>
//       Returns None if empty. Sum divided by count.
//
//     pub fn median(v: &[f64]) -> Option<f64>
//       Returns None if empty. Middle value of sorted copy.
//       For even length, return the lower middle.
//
//     pub fn mode(v: &[i32]) -> Option<i32>
//       Returns None if empty. Most frequently occurring value.
//       Hint: use a HashMap to count occurrences.
// }
//
// In main: call all three on sample data. sorted_copy must NOT be callable from main.
//
// Expected output (approximately):
//   mean of [1,2,3,4,5]: Some(3.0)
//   median of [1,2,3,4,5]: Some(3.0)
//   median of [1,2,3,4]: Some(2.0)
//   mode of [1,2,2,3,3,3]: Some(3)
//   mean of []: None
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement the stats module above main, then use it here
}
