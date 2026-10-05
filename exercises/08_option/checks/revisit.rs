#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_age() {
        assert_eq!(parse_age("0"), Some(0));
        assert_eq!(parse_age("150"), Some(150));
        assert_eq!(parse_age("151"), None);
        assert_eq!(parse_age("abc"), None);
        assert_eq!(parse_age(""), None);
        assert_eq!(parse_age(" 25 "), None);
    }

    #[test]
    fn check_greet() {
        assert_eq!(greet(Some("Alice")), "Hello, Alice!");
        assert_eq!(greet(None), "Hello, stranger!");
        assert_eq!(greet(Some("")), "Hello, !");
    }

    #[test]
    fn check_find_matching() {
        let v = vec!["rust".into(), "ruby".into(), "go".into()];
        assert_eq!(find_matching(&v, "ru").map(String::as_str), Some("rust"));
        assert_eq!(find_matching(&v, "py"), None);
        assert_eq!(find_matching(&v, "").map(String::as_str), Some("rust"));
        assert_eq!(find_matching(&[], "ru"), None);
    }
}
