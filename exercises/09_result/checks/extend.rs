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

    #[test]
    fn check_parse_and_grade() {
        assert!(matches!(parse_and_grade("90"), Ok("A")));
        assert!(matches!(parse_and_grade("80"), Ok("B")));
        assert!(matches!(parse_and_grade("70"), Ok("C")));
        assert!(matches!(parse_and_grade("60"), Ok("D")));
        assert!(matches!(parse_and_grade("59"), Ok("F")));
        assert!(matches!(
            parse_and_grade("abc"),
            Err(ScoreError::NotANumber)
        ));
    }
}
