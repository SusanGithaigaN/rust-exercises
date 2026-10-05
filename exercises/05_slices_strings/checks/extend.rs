#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_word() {
        assert_eq!(first_word("the quick"), "the");
        assert_eq!(first_word("héllo world"), "héllo");
        assert_eq!(first_word("one"), "one");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word(" hi"), "");
    }

    #[test]
    fn check_last_word() {
        assert_eq!(last_word("the quick fox"), "fox");
        assert_eq!(last_word("one"), "one");
        assert_eq!(last_word(""), "");
        assert_eq!(last_word("hi "), "");
    }

    #[test]
    fn check_word_count() {
        assert_eq!(
            word_count(
                "  a	b
 c  "
            ),
            3
        );
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   "), 0);
    }

    #[test]
    fn check_reverse_words() {
        assert_eq!(reverse_words("the quick fox"), "fox quick the");
        assert_eq!(reverse_words("  a	b  "), "b a");
        assert_eq!(reverse_words(""), "");
    }
}
