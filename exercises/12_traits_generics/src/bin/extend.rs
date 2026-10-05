// Read ../../README.md before starting.

trait Describable {
    fn describe(&self) -> String;
    fn short_description(&self) -> String {
        let d = self.describe();
        if d.chars().count() > 30 {
            format!("{}...", d.chars().take(30).collect::<String>())
        } else {
            d
        }
    }
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}
struct Movie {
    title: String,
    director: String,
    year: u16,
}

impl Describable for Book {
    fn describe(&self) -> String {
        todo!()
    }
}

impl Describable for Movie {
    fn describe(&self) -> String {
        todo!()
    }
}

fn print_description(item: &impl Describable) {
    todo!()
}

trait HasTitle {
    fn title(&self) -> &str;
}

impl HasTitle for Book {
    fn title(&self) -> &str {
        todo!()
    }
}

impl HasTitle for Movie {
    fn title(&self) -> &str {
        todo!()
    }
}

fn find_by_title<'a, T: HasTitle>(items: &'a [T], title: &str) -> Option<&'a T> {
    todo!()
}

fn main() {
    let books = vec![
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            pages: 456,
        },
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik"),
            pages: 560,
        },
    ];
    let movies = vec![Movie {
        title: String::from("Dune"),
        director: String::from("Denis Villeneuve"),
        year: 2021,
    }];

    print_description(&books[0]);

    println!(
        "{:?}",
        find_by_title(&books, "Rust in Action").map(|b| b.title())
    );
    println!("{:?}", find_by_title(&books, "Missing").map(|b| b.title()));
    println!("{:?}", find_by_title(&movies, "Dune").map(|m| m.title()));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
