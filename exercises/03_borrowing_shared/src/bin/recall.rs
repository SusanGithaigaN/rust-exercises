// Read ../../README.md before starting.

fn char_count(s: &String) -> usize {
    todo!()
}

fn starts_with_vowel(s: &String) -> bool {
    todo!()
}

fn count_spaces(s: &String) -> usize {
    todo!()
}

fn main() {
    let s = String::from("open the door");
    println!("{}", char_count(&s));
    println!("{}", starts_with_vowel(&s));
    println!("{}", count_spaces(&s));
    println!("{}", s);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
