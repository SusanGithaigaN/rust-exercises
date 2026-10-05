// TOPIC 05 — Slices and Strings
// EXERCISE TYPE: Extend
// =============================================
// The two recall functions are provided. Add two more.
//
// ADD:
// 1. fn word_count(s: &str) -> usize
//    Returns the number of words (split by whitespace).
//    Hint: s.split_whitespace().count()
//
// 2. fn reverse_words(s: &str) -> String
//    Reverses the ORDER of words (not the characters within each word).
//    "the quick brown fox" -> "fox brown quick the"
//    Hint: s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
//
// Expected output:
//   word count of "the quick brown fox": 4
//   word count of "  spaces   here  ": 2
//   reversed: "fox brown quick the"
//   reversed single word: "onlyone"
//
// Run: cargo run --bin extend

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[0..i],
        None => s,
    }
}

fn last_word(s: &str) -> &str {
    match s.rfind(' ') {
        Some(i) => &s[i + 1..],
        None => s,
    }
}

// TODO: add word_count and reverse_words here

fn main() {
    let sentence = "the quick brown fox";
    println!("first: {}", first_word(sentence));
    println!("last: {}", last_word(sentence));
    // TODO: call word_count and reverse_words with the expected values above
}
