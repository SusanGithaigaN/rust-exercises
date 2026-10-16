// Read ../../README.md before starting.
// Use iterator methods only — no index loops.

// Count words longer than 5 characters.
fn longer_than_five(words: &[&str]) -> usize {
    todo!()
}
// Collect words that start with 'a' or 'b' (lowercase) into Strings.
fn starts_ab(words: &[&str]) -> Vec<String> {
    todo!()
}
// Return the longest word. If a tie, return the first. Panics on empty slice.
fn longest<'a>(words: &[&'a str]) -> &'a str {
    todo!()
}
// Join all words with ", ". Empty slice → "".
fn joined(words: &[&str]) -> String {
    todo!()
}
// Collect the first character of each word.
fn first_letters(words: &[&str]) -> Vec<char> {
    todo!()
}
// Count words that contain the letter 'e'.
fn contains_e(words: &[&str]) -> usize {
    todo!()
}

fn main() {
    let words = vec!["apple", "banana", "cherry", "avocado", "blueberry", "fig"];
    println!("{}", longer_than_five(&words));    // 3
    println!("{:?}", starts_ab(&words));         // ["apple", "banana", "avocado", "blueberry"]
    println!("{}", longest(&words));             // blueberry
    println!("{}", joined(&words));              // apple, banana, cherry, avocado, blueberry, fig
    println!("{:?}", first_letters(&words));     // ['a', 'b', 'c', 'a', 'b', 'f']
    println!("{}", contains_e(&words));          // 4
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
