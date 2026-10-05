// TOPIC 05 — Slices and Strings
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK A: fn truncate_words(s: &str, max_words: usize) -> String
//   Returns a String containing at most max_words words from s.
//   If s has fewer words, return all of them.
//   "the quick brown fox" with max_words=2 -> "the quick"
//
// TASK B: fn contains_word(text: &str, word: &str) -> bool
//   Returns true if text contains word as a whole word (surrounded by spaces or at ends).
//   "hello world" contains "world": true
//   "hello worldwide" contains "world": false
//   Hint: use split_whitespace and check for equality
//
// Expected output:
//   truncate "the quick brown fox" to 2: "the quick"
//   truncate "short" to 5: "short"
//   "hello world" contains "world": true
//   "hello worldwide" contains "world": false
//   "world peace" contains "world": true
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement truncate_words and contains_word above main, then call here
}
