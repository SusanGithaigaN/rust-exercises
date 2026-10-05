// TOPIC 12 — Traits and Generics (Basic)
// EXERCISE TYPE: Recall
// =============================================
// Define a trait and implement it for two different structs.
//
// Step 1: Define this trait:
//   trait Describable {
//       fn describe(&self) -> String;
//       fn short_description(&self) -> String {
//           // default impl: first 30 chars of describe(), with "..." if truncated
//       }
//   }
//
// Step 2: Define these structs and implement Describable for each:
//   struct Book { title: String, author: String, pages: u32 }
//   -> describe: "Book: '{title}' by {author}, {pages} pages"
//
//   struct Movie { title: String, director: String, year: u16 }
//   -> describe: "Movie: '{title}' directed by {director} ({year})"
//
// Step 3: Write a generic function:
//   fn print_description(item: &impl Describable)
//   Prints both describe() and short_description().
//
// Expected output:
//   Book: 'The Rust Programming Language' by Steve Klabnik, 560 pages
//   Book: 'The Rust Programming L...
//   Movie: 'Inception' directed by Christopher Nolan (2010)
//   Movie: 'Inception' directed b...
//
// Run: cargo run --bin recall

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
    // TODO: define trait, structs, impls, and print_description — then call it here:
    print_description(&book);
    print_description(&movie);
}
