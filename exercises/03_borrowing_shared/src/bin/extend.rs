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

fn count_vowels(s: &str) -> usize {
    todo!()
}

fn is_palindrome(s: &str) -> bool {
    todo!()
}

fn main() {
    let s = String::from("open the door");
    println!("{}", char_count(&s));
    println!("{}", starts_with_vowel(&s));
    println!("{}", count_spaces(&s));
    println!("{}", count_vowels("hello world"));
    println!("{}", count_vowels("rhythm"));
    println!("{}", is_palindrome("racecar"));
    println!("{}", is_palindrome("hello"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
