// Read ../../README.md before starting.

fn first_word(s: &str) -> &str {
    todo!()
}

fn last_word(s: &str) -> &str {
    todo!()
}

fn main() {
    let owned = String::from("the quick brown fox");
    println!("{}", first_word(&owned));
    println!("{}", last_word(&owned));
    println!("{}", first_word("onlyone"));
    println!("{}", last_word("onlyone"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
