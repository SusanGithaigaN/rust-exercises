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

fn parse_and_grade(s: &str) -> Result<&'static str, ScoreError> {
    todo!()
}

fn main() {
    println!("{:?}", parse_score("85"));
    println!("{:?}", parse_score("abc"));
    println!("{:?}", parse_and_grade("95"));
    println!("{:?}", parse_and_grade("72"));
    println!("{:?}", parse_and_grade("55"));
    println!("{:?}", parse_and_grade("abc"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
