// Read ../../README.md before starting.

fn truncate_words(s: &str, max_words: usize) -> String {
    todo!()
}

fn contains_word(text: &str, word: &str) -> bool {
    todo!()
}

fn main() {
    println!("{}", truncate_words("the quick brown fox", 2));
    println!("{}", truncate_words("short", 5));
    println!("{}", contains_word("hello world", "world"));
    println!("{}", contains_word("hello worldwide", "world"));
    println!("{}", contains_word("world peace", "world"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
