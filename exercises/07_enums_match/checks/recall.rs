#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_opposite() {
        assert!(matches!(opposite(Direction::North), Direction::South));
        assert!(matches!(opposite(Direction::South), Direction::North));
        assert!(matches!(opposite(Direction::East), Direction::West));
        assert!(matches!(opposite(Direction::West), Direction::East));
    }

    #[test]
    fn check_is_horizontal() {
        assert!(is_horizontal(&Direction::East));
        assert!(is_horizontal(&Direction::West));
        assert!(!is_horizontal(&Direction::North));
        assert!(!is_horizontal(&Direction::South));
    }

    #[test]
    fn check_describe() {
        assert_eq!(describe(&Direction::North), "heading north");
        assert_eq!(describe(&Direction::South), "heading south");
        assert_eq!(describe(&Direction::East), "heading east");
        assert_eq!(describe(&Direction::West), "heading west");
    }

    #[test]
    fn check_value_in_cents() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter), 25);
    }
}
