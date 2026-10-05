// Read ../../README.md before starting.

#[derive(Debug)]
enum ScoreError {
    EmptyInput,
    NotANumber,
    OutOfRange,
}

fn parse_score(s: &str) -> Result<u32, ScoreError> {
    todo!()
}

fn main() {
    println!("{:?}", parse_score(""));
    println!("{:?}", parse_score("abc"));
    println!("{:?}", parse_score("150"));
    println!("{:?}", parse_score("85"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
