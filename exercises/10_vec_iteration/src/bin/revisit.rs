// Read ../../README.md before starting.

fn longer_than_five(words: &[&str]) -> usize {
    todo!()
}
fn starts_ab(words: &[&str]) -> Vec<String> {
    todo!()
}
fn longest<'a>(words: &[&'a str]) -> &'a str {
    todo!()
}
fn joined(words: &[&str]) -> String {
    todo!()
}
fn first_letters(words: &[&str]) -> Vec<char> {
    todo!()
}
fn contains_e(words: &[&str]) -> usize {
    todo!()
}

fn main() {
    let words = vec!["apple", "banana", "cherry", "avocado", "blueberry", "fig"];
    println!("{}", longer_than_five(&words));
    println!("{:?}", starts_ab(&words));
    println!("{}", longest(&words));
    println!("{}", joined(&words));
    println!("{:?}", first_letters(&words));
    println!("{}", contains_e(&words));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
