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

    #[test]
    fn check_title() {
        let b = Book {
            title: "Rust".into(),
            author: "A".into(),
            pages: 1,
        };
        assert_eq!(b.title(), "Rust");
        let m = Movie {
            title: "Dune".into(),
            director: "D".into(),
            year: 2021,
        };
        assert_eq!(m.title(), "Dune");
    }

    #[test]
    fn check_find_by_title() {
        let b = vec![
            Book {
                title: "Rust".into(),
                author: "A".into(),
                pages: 1,
            },
            Book {
                title: "Rust".into(),
                author: "B".into(),
                pages: 2,
            },
        ];
        assert_eq!(find_by_title(&b, "Rust").map(|b| b.pages), Some(1));
        assert!(find_by_title(&b, "rust").is_none());
        assert!(find_by_title::<Book>(&[], "Rust").is_none());
    }
}
