#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_validate_name() {
        assert!(matches!(validate_name(""), Err(ValidationError::NameEmpty)));
        assert!(matches!(validate_name("Alice"), Ok("Alice")));
        let name = "é".repeat(50);
        assert!(validate_name(&name).is_ok());
        assert!(matches!(
            validate_name(&"a".repeat(51)),
            Err(ValidationError::NameTooLong)
        ));
    }

    #[test]
    fn check_validate_age() {
        assert!(matches!(
            validate_age("abc"),
            Err(ValidationError::AgeNotANumber)
        ));
        assert!(matches!(
            validate_age("17"),
            Err(ValidationError::AgeTooYoung)
        ));
        assert!(matches!(
            validate_age("121"),
            Err(ValidationError::AgeTooOld)
        ));
        assert!(matches!(validate_age("18"), Ok(18)));
        assert!(matches!(validate_age("120"), Ok(120)));
    }

    #[test]
    fn check_create_profile() {
        assert!(matches!(create_profile("Alice","30"),Ok(s) if s == "Name: Alice, Age: 30"));
        assert!(matches!(
            create_profile("", "abc"),
            Err(ValidationError::NameEmpty)
        ));
        assert!(matches!(
            create_profile("Alice", "15"),
            Err(ValidationError::AgeTooYoung)
        ));
    }
}
