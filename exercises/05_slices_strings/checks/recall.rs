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
}
