// Read ../../README.md before starting.

// Return at most the first max_words words joined by single spaces. Limit 0 or empty input → "".
fn truncate_words(s: &str, max_words: usize) -> String {
    todo!()
}

// True only if a whitespace-delimited token exactly equals word. "worldwide" does not contain "world". Empty word → false.
fn contains_word(text: &str, word: &str) -> bool {
    todo!()
}

fn main() {
    println!("{}", truncate_words("the quick brown fox", 2)); // the quick
    println!("{}", truncate_words("short", 5));               // short
    println!("{}", contains_word("hello world", "world"));    // true
    println!("{}", contains_word("hello worldwide", "world")); // false
    println!("{}", contains_word("world peace", "world"));    // true
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
