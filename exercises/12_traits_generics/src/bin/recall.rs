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

// Return "Book: '<title>' by <author>, <pages> pages".
impl Describable for Book {
    fn describe(&self) -> String {
        todo!()
    }
}

// Return "Movie: '<title>' directed by <director> (<year>)".
impl Describable for Movie {
    fn describe(&self) -> String {
        todo!()
    }
}

// Print describe() on one line, then short_description() on the next.
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
    // Book: 'The Rust Programming Language' by Steve Klabnik, 560 pages
    // Book: 'The Rust Programming L...
    print_description(&movie);
    // Movie: 'Inception' directed by Christopher Nolan (2010)
    // Movie: 'Inception' directed b...
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
