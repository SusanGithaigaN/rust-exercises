// TOPIC 12 — Traits and Generics
// EXERCISE TYPE: Extend
// =============================================
// Book and Movie with Describable are provided. Add a second trait and a generic function.
//
// ADD:
// trait HasTitle {
//     fn title(&self) -> &str;
// }
// Implement HasTitle for Book and Movie.
//
// ADD a generic function:
// fn find_by_title<T: HasTitle>(items: &[T], title: &str) -> Option<&T>
//   Returns the first item whose title matches (case-sensitive).
//
// Expected output:
//   Found book: Book: 'Rust in Action' by Tim McNamara, 456 pages
//   Not found: None
//   Found movie: Movie: 'Dune' directed by Denis Villeneuve (2021)
//
// Run: cargo run --bin extend

trait Describable {
    fn describe(&self) -> String;
    fn short_description(&self) -> String {
        let d = self.describe();
        if d.len() > 30 { format!("{}...", &d[..30]) } else { d }
    }
}

struct Book { title: String, author: String, pages: u32 }
struct Movie { title: String, director: String, year: u16 }

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: '{}' by {}, {} pages", self.title, self.author, self.pages)
    }
}

impl Describable for Movie {
    fn describe(&self) -> String {
        format!("Movie: '{}' directed by {} ({})", self.title, self.director, self.year)
    }
}

// TODO: add HasTitle trait, implement for Book and Movie
// TODO: add find_by_title function

fn main() {
    let books = vec![
        Book { title: String::from("Rust in Action"), author: String::from("Tim McNamara"), pages: 456 },
        Book { title: String::from("The Rust Programming Language"), author: String::from("Steve Klabnik"), pages: 560 },
    ];
    let movies = vec![
        Movie { title: String::from("Dune"), director: String::from("Denis Villeneuve"), year: 2021 },
        Movie { title: String::from("Inception"), director: String::from("Christopher Nolan"), year: 2010 },
    ];
    // TODO: test find_by_title here
    for b in &books { println!("{}", b.describe()); }
}
