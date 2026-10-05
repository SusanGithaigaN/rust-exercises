// Read ../../README.md before starting.

fn append_exclamation(s: String) -> String {
    todo!()
}

fn shout(s: String) -> String {
    todo!()
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
