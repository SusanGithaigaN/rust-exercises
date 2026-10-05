#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_longer_than_five() {
        assert_eq!(
            longer_than_five(&["apple", "banana", "cherry", "avocado", "blueberry", "fig"]),
            4
        );
        assert_eq!(longer_than_five(&[]), 0);
    }

    #[test]
    fn check_starts_ab() {
        assert_eq!(
            starts_ab(&["apple", "banana", "cherry", "", "Apple"]),
            vec!["apple", "banana"]
        );
        assert!(starts_ab(&[]).is_empty());
    }

    #[test]
    fn check_longest() {
        assert_eq!(longest(&["a", "long", "also"]), "long");
        assert_eq!(longest(&["ééé", "abcd"]), "abcd");
        assert_eq!(longest(&[]), "");
    }

    #[test]
    fn check_joined() {
        assert_eq!(joined(&["apple", "banana", "fig"]), "apple, banana, fig");
        assert_eq!(joined(&[]), "");
    }

    #[test]
    fn check_first_letters() {
        assert_eq!(first_letters(&["apple", "", "éclair"]), vec!['a', 'é']);
        assert!(first_letters(&[]).is_empty());
    }

    #[test]
    fn check_contains_e() {
        assert_eq!(
            contains_e(&["apple", "banana", "cherry", "avocado", "blueberry", "fig"]),
            3
        );
        assert_eq!(contains_e(&["eee", "E", ""]), 1);
        assert_eq!(contains_e(&[]), 0);
    }
}
