// TOPIC 03 — Borrowing: Shared References
// EXERCISE TYPE: Recall
// =============================================
// Write three functions that borrow a String without taking ownership.
// After each call, the original String must still be usable.
//
// 1. fn char_count(s: &String) -> usize
//    Returns the number of characters.
//
// 2. fn starts_with_vowel(s: &String) -> bool
//    Returns true if the first character is a, e, i, o, or u (lowercase).
//    Hint: s.chars().next()
//
// 3. fn count_spaces(s: &String) -> usize
//    Returns the number of space characters.
//    Hint: s.chars().filter(...)
//
// In main:
//   - Create ONE String
//   - Call all three functions on it
//   - After all calls, print the original string to prove it still exists
//
// Expected output (using "open the door"):
//   chars: 13
//   starts with vowel: true
//   spaces: 2
//   original still here: open the door
//
// Run: cargo run --bin recall

fn main() {
    // TODO: implement the three functions above, then use them here
}
