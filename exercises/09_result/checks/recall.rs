#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_score() {
        assert!(matches!(parse_score(""), Err(ScoreError::EmptyInput)));
        assert!(matches!(parse_score("abc"), Err(ScoreError::NotANumber)));
        assert!(matches!(parse_score("-1"), Err(ScoreError::NotANumber)));
        assert!(matches!(parse_score("101"), Err(ScoreError::OutOfRange)));
        assert!(matches!(parse_score("0"), Ok(0)));
        assert!(matches!(parse_score("100"), Ok(100)));
    }
}
