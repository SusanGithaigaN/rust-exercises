// Read ../../README.md before starting.

#[derive(Debug)]
enum ScoreError {
    EmptyInput,
    NotANumber,
    OutOfRange,
}

// Parse s as a score (0–100). Empty → EmptyInput, non-numeric → NotANumber, >100 → OutOfRange.
fn parse_score(s: &str) -> Result<u32, ScoreError> {
    todo!()
}

fn main() {
    println!("{:?}", parse_score(""));    // Err(EmptyInput)
    println!("{:?}", parse_score("abc")); // Err(NotANumber)
    println!("{:?}", parse_score("150")); // Err(OutOfRange)
    println!("{:?}", parse_score("85"));  // Ok(85)
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
