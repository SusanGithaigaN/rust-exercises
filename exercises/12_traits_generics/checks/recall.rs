#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_book() {
        let b = Book {
            title: "Rust".into(),
            author: "Alice".into(),
            pages: 100,
        };
        assert_eq!(b.describe(), "Book: 'Rust' by Alice, 100 pages");
        assert_eq!(b.short_description(), "Book: 'Rust' by Alice, 100 pag...");
    }

    #[test]
    fn check_movie() {
        let m = Movie {
            title: "Dune".into(),
            director: "Denis".into(),
            year: 2021,
        };
        assert_eq!(m.describe(), "Movie: 'Dune' directed by Denis (2021)");
    }

    #[test]
    fn check_print_description() {
        let b = Book {
            title: "é".repeat(40),
            author: "A".into(),
            pages: 1,
        };
        let full = b.describe();
        assert_eq!(
            b.short_description(),
            format!("{}...", full.chars().take(30).collect::<String>())
        );
        print_description(&b);
    }
}
