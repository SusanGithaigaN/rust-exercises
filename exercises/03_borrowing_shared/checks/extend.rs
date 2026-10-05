#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_char_count() {
        let s = String::from("hé");
        assert_eq!(char_count(&s), 2);
        assert_eq!(s, "hé");
        assert_eq!(char_count(&String::new()), 0);
    }

    #[test]
    fn check_starts_with_vowel() {
        assert!(starts_with_vowel(&"apple".into()));
        assert!(!starts_with_vowel(&"Apple".into()));
        assert!(!starts_with_vowel(&String::new()));
        assert!(!starts_with_vowel(&"rust".into()));
    }

    #[test]
    fn check_count_spaces() {
        assert_eq!(
            count_spaces(
                &"a  b	
"
                .into()
            ),
            2
        );
        assert_eq!(count_spaces(&String::new()), 0);
    }

    #[test]
    fn check_count_vowels() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels("AEIOU"), 0);
        assert_eq!(count_vowels(""), 0);
    }

    #[test]
    fn check_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("été"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("Racecar"));
        assert!(!is_palindrome("hello"));
    }
}
