#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_create_entry() {
        assert_eq!(
            create_entry("2024-01-01".into(), "Server started".into()),
            "[2024-01-01] Server started"
        );
        assert_eq!(create_entry(String::new(), String::new()), "[] ");
    }

    #[test]
    fn check_prefix_entry() {
        assert_eq!(
            prefix_entry("INFO".into(), "[today] Ready".into()),
            "INFO: [today] Ready"
        );
        assert_eq!(prefix_entry(String::new(), String::new()), ": ");
    }
}
