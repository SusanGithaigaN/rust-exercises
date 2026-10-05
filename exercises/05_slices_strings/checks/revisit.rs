#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_truncate_words() {
        assert_eq!(truncate_words("the quick brown fox", 2), "the quick");
        assert_eq!(truncate_words(" short ", 5), "short");
        assert_eq!(truncate_words("hello", 0), "");
        assert_eq!(truncate_words("", 2), "");
    }

    #[test]
    fn check_contains_word() {
        assert!(contains_word("hello world", "world"));
        assert!(!contains_word("hello worldwide", "world"));
        assert!(!contains_word("world!", "world"));
        assert!(!contains_word("", ""));
    }
}
