#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_append_exclamation() {
        assert_eq!(append_exclamation(String::from("hello")), "hello!");
        assert_eq!(append_exclamation(String::new()), "!");
    }

    #[test]
    fn check_shout() {
        assert_eq!(shout(String::from("hello!")), "HELLO!");
        assert_eq!(shout(String::new()), "");
    }
}
