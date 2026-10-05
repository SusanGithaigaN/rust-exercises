// TOPIC 05 — Slices and Strings
// EXERCISE TYPE: Recall
// =============================================
// Write two functions that accept &str (not &String).
// Both should work when called with a String reference OR a string literal.
//
// 1. fn first_word(s: &str) -> &str
//    Returns a slice of the first word (everything before the first space).
//    If there is no space, return the whole string.
//    Hint: s.find(' ') returns Option<usize>
//
// 2. fn last_word(s: &str) -> &str
//    Returns a slice of the last word (everything after the last space).
//    If there is no space, return the whole string.
//    Hint: s.rfind(' ')
//
// In main:
//   - Call both functions with a String (pass as &owned_string or &owned_string[..])
//   - Call both functions with a string literal directly
//   - Both should compile and produce the same type of result
//
// Expected output:
//   first word of "the quick brown fox": the
//   last word of "the quick brown fox": fox
//   first word of "onlyone": onlyone
//   last word of "onlyone": onlyone
//
// Run: cargo run --bin recall

fn main() {
    // TODO: implement the two functions above, then use them here
}
