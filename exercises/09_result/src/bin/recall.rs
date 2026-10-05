// TOPIC 09 — Result<T, E>
// EXERCISE TYPE: Recall
// =============================================
// Define a custom error type and a function that returns Result.
//
// Step 1: Define this enum:
//   #[derive(Debug)]
//   enum ScoreError { EmptyInput, NotANumber, OutOfRange }
//
// Step 2: Write this function:
//   fn parse_score(s: &str) -> Result<u32, ScoreError>
//   - Returns Err(ScoreError::EmptyInput) if s is empty
//   - Returns Err(ScoreError::NotANumber) if s does not parse to a number
//     Hint: s.parse::<u32>().map_err(|_| ScoreError::NotANumber)
//   - Returns Err(ScoreError::OutOfRange) if the number > 100
//   - Returns Ok(n) otherwise
//
// In main: call parse_score with "", "abc", "150", and "85".
// Handle each result with match and print a clear message.
//
// Expected output:
//   "": EmptyInput
//   "abc": NotANumber
//   "150": OutOfRange
//   "85": score is 85
//
// Run: cargo run --bin recall

fn main() {
    // TODO: define ScoreError, implement parse_score, then use them here
}
