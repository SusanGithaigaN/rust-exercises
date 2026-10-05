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

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik"),
        pages: 560,
    };
    let movie = Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
        year: 2010,
    };
    print_description(&book);
    print_description(&movie);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
