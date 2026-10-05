#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_analyze() {
        assert_eq!(analyze("Hi. Ready?"), (2, 10, 2, true));
        assert_eq!(analyze(""), (0, 0, 0, false));
        assert_eq!(analyze("hé!!"), (1, 4, 2, false));
    }
}
