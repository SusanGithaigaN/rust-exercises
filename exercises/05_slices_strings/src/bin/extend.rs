// Read ../../README.md before starting.

fn first_word(s: &str) -> &str {
    todo!()
}
fn last_word(s: &str) -> &str {
    todo!()
}

fn word_count(s: &str) -> usize {
    todo!()
}

fn reverse_words(s: &str) -> String {
    todo!()
}

fn main() {
    let s = "the quick brown fox";
    println!("{}", first_word(s));
    println!("{}", last_word(s));
    println!("{}", word_count(s));
    println!("{}", word_count("  a  b  "));
    println!("{}", reverse_words(s));
    println!("{}", reverse_words("one"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
