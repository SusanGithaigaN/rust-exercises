// Read ../../README.md before starting.

fn analyze(text: &str) -> (usize, usize, usize, bool) {
    todo!()
}

fn main() {
    let s = "Hello world. How are you?";
    let (words, chars, sentences, question) = analyze(s);
    println!("words: {}", words);
    println!("chars: {}", chars);
    println!("sentences: {}", sentences);
    println!("question: {}", question);
    println!("{}", s);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
