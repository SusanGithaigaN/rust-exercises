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

// Call parse_score with ?, then map the score: ≥90→"A", ≥80→"B", ≥70→"C", ≥60→"D", else "F".
fn parse_and_grade(s: &str) -> Result<&'static str, ScoreError> {
    todo!()
}

fn main() {
    println!("{:?}", parse_score("85"));     // Ok(85)
    println!("{:?}", parse_score("abc"));    // Err(NotANumber)
    println!("{:?}", parse_and_grade("95")); // Ok("A")
    println!("{:?}", parse_and_grade("72")); // Ok("C")
    println!("{:?}", parse_and_grade("55")); // Ok("F")
    println!("{:?}", parse_and_grade("abc")); // Err(NotANumber)
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
