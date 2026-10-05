// TOPIC 09 — Result<T, E>
// EXERCISE TYPE: Extend
// =============================================
// parse_score from recall is provided. Add a function that chains on top of it.
//
// ADD:
// fn parse_and_grade(s: &str) -> Result<&'static str, ScoreError>
//   Calls parse_score(s) using ? to propagate errors.
//   If Ok, converts the score to a letter grade:
//     >= 90 -> "A"
//     >= 80 -> "B"
//     >= 70 -> "C"
//     >= 60 -> "D"
//     < 60  -> "F"
//
// Expected output:
//   "95" -> Ok("A")
//   "72" -> Ok("C")
//   "55" -> Ok("F")
//   "abc" -> Err(NotANumber)
//   "" -> Err(EmptyInput)
//   "150" -> Err(OutOfRange)
//
// Run: cargo run --bin extend

#[derive(Debug)]
enum ScoreError { EmptyInput, NotANumber, OutOfRange }

fn parse_score(s: &str) -> Result<u32, ScoreError> {
    if s.is_empty() { return Err(ScoreError::EmptyInput); }
    let n = s.parse::<u32>().map_err(|_| ScoreError::NotANumber)?;
    if n > 100 { return Err(ScoreError::OutOfRange); }
    Ok(n)
}

// TODO: add parse_and_grade here

fn main() {
    // Test parse_score
    for input in ["85", "", "abc", "150"] {
        println!("{:?}: {:?}", input, parse_score(input));
    }
    // TODO: test parse_and_grade here
}
