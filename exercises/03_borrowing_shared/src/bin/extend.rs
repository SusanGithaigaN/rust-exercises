// TOPIC 03 — Borrowing: Shared References
// EXERCISE TYPE: Extend
// =============================================
// The three recall functions are provided. Add two more that use &str
// instead of &String — and notice the difference.
//
// ADD:
// 1. fn count_vowels(s: &str) -> usize
//    Counts lowercase vowels: a, e, i, o, u.
//    Hint: s.chars().filter(|c| "aeiou".contains(*c)).count()
//
// 2. fn is_palindrome(s: &str) -> bool
//    Returns true if s reads the same forwards and backwards.
//    Hint: s.chars().rev().collect::<String>() == s
//
// Notice: your new functions take &str, not &String.
// Try passing a &String to them — does it work? Why?
//
// Expected output:
//   vowels in "hello world": 3
//   vowels in "rhythm": 0
//   "racecar" is palindrome: true
//   "hello" is palindrome: false
//
// Run: cargo run --bin extend

fn char_count(s: &String) -> usize {
    s.chars().count()
}

fn starts_with_vowel(s: &String) -> bool {
    matches!(s.chars().next(), Some('a' | 'e' | 'i' | 'o' | 'u'))
}

fn count_spaces(s: &String) -> usize {
    s.chars().filter(|&c| c == ' ').count()
}

// TODO: add count_vowels and is_palindrome here

fn main() {
    let s = String::from("open the door");
    println!("chars: {}", char_count(&s));
    println!("starts with vowel: {}", starts_with_vowel(&s));
    println!("spaces: {}", count_spaces(&s));
    // TODO: call count_vowels and is_palindrome with the expected values above
}
