// Read ../../README.md before starting.

// Return the slice of s before the first space. No space → return all of s. Leading space or empty → return "".
fn first_word(s: &str) -> &str {
    todo!()
}

// Return the slice of s after the last space. No space → return all of s. Trailing space or empty → return "".
fn last_word(s: &str) -> &str {
    todo!()
}

fn main() {
    let owned = String::from("the quick brown fox");
    println!("{}", first_word(&owned)); // the
    println!("{}", last_word(&owned));  // fox
    println!("{}", first_word("onlyone")); // onlyone
    println!("{}", last_word("onlyone"));  // onlyone
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
