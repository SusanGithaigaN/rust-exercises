// Read ../../README.md before starting.

fn append_exclamation(mut s: String) -> String {
    s.push_str("!");
    s
}

fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    let s = String::from("hello");
    let s = append_exclamation(s);
    println!("{}", s);
    println!("{}", shout(s));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
