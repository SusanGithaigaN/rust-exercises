// Read ../../README.md before starting.

fn char_count(s: &String) -> usize {
    // todo!()
    s.chars().count()
}
fn starts_with_vowel(s: &String) -> bool {
    // todo!()
    for character in s.chars() {
        return character.is_lowercase();
    }
    false
}
fn count_spaces(s: &String) -> usize {
    // todo!()
    s.chars().filter(|&s| s == ' ').count()
}

fn count_vowels(s: &str) -> usize {
    // todo!()
    let mut count = 0;
    for ch in s.chars() {
        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
            count += 1;
        }
    }
    count
}

fn is_palindrome(s: &str) -> bool {
    // todo!()
    let s_lower = s.to_ascii_lowercase();
    s_lower.chars().eq(s_lower.chars().rev())
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
