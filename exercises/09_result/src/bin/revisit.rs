// TOPIC 09 — Result<T, E>
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Build a small user profile validator.
//
// #[derive(Debug)]
// enum ValidationError { NameEmpty, NameTooLong, AgeTooYoung, AgeTooOld, AgeNotANumber }
//
// fn validate_name(name: &str) -> Result<&str, ValidationError>
//   Err(NameEmpty) if empty, Err(NameTooLong) if > 50 chars, Ok(name) otherwise.
//
// fn validate_age(age_str: &str) -> Result<u32, ValidationError>
//   Err(AgeNotANumber) if parse fails, Err(AgeTooYoung) if < 18,
//   Err(AgeTooOld) if > 120, Ok(age) otherwise.
//
// fn create_profile(name: &str, age_str: &str) -> Result<String, ValidationError>
//   Uses ? to chain both validators.
//   Returns Ok("Name: {name}, Age: {age}") if both are valid.
//
// Expected output:
//   ("Alice", "30"): Ok("Name: Alice, Age: 30")
//   ("", "30"): Err(NameEmpty)
//   ("Alice", "abc"): Err(AgeNotANumber)
//   ("Alice", "15"): Err(AgeTooYoung)
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement all types and functions above main, then test here
}
