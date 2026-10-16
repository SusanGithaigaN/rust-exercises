// Read ../../README.md before starting.

// Return the slice of s before the first space. No space → all of s. Leading space or empty → "".
fn first_word(s: &str) -> &str {
    todo!()
}
// Return the slice of s after the last space. No space → all of s. Trailing space or empty → "".
fn last_word(s: &str) -> &str {
    todo!()
}

// Count whitespace-separated words; ignore repeated or surrounding whitespace. Empty → 0.
fn word_count(s: &str) -> usize {
    todo!()
}

// Reverse the word order with one space between each. "the quick fox" → "fox quick the". Empty → "".
fn reverse_words(s: &str) -> String {
    todo!()
}

fn main() {
    let s = "the quick brown fox";
    println!("{}", first_word(s));        // the
    println!("{}", last_word(s));         // fox
    println!("{}", word_count(s));        // 4
    println!("{}", word_count("  a  b  ")); // 2
    println!("{}", reverse_words(s));     // fox brown quick the
    println!("{}", reverse_words("one")); // one
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
