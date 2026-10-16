// Read ../../README.md before starting.

// Parse a valid age (0–150). Return None if not a number or out of range.
fn parse_age(s: &str) -> Option<u32> {
    todo!()
}

// Some("Alice") → "Hello, Alice!". None → "Hello, stranger!".
fn greet(name: Option<&str>) -> String {
    todo!()
}

// Return the first string in v that starts with prefix, or None if none matches.
fn find_matching<'a>(v: &'a [String], prefix: &str) -> Option<&'a String> {
    todo!()
}

fn main() {
    println!("{:?}", parse_age("25"));  // Some(25)
    println!("{:?}", parse_age("abc")); // None
    println!("{:?}", parse_age("200")); // None
    println!("{}", greet(Some("Alice"))); // Hello, Alice!
    println!("{}", greet(None));          // Hello, stranger!

    let words = vec![
        String::from("rust"),
        String::from("ruby"),
        String::from("go"),
    ];
    println!("{:?}", find_matching(&words, "ru")); // Some("rust")
    println!("{:?}", find_matching(&words, "py")); // None
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
