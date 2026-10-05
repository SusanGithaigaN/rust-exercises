// TOPIC 04 — Borrowing: Mutable References
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: fn remove_duplicates(v: &mut Vec<i32>)
//   Removes duplicate values from v in place, keeping the FIRST occurrence.
//   [3, 1, 4, 1, 5, 9, 2, 6, 5, 3] -> [3, 1, 4, 5, 9, 2, 6]
//   Order of remaining elements must be preserved.
//
//   Approach (one option):
//     - Use a HashSet to track seen values
//     - Retain only elements not yet seen
//     Hint: v.retain(|x| seen.insert(*x))
//     (HashSet::insert returns false if the value was already present)
//
// Also write: fn normalize(v: &mut Vec<f64>)
//   Divides every element by the maximum value so all values are in [0.0, 1.0].
//   If the Vec is empty or max is 0.0, do nothing.
//
// Expected output:
//   after remove_duplicates: [3, 1, 4, 5, 9, 2, 6]
//   after normalize: [0.333..., 1.0, 0.666...]  (approximately)
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement remove_duplicates and normalize above main, then call here
}
