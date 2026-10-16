// Read ../../README.md before starting.

#[derive(Debug)]
enum ValidationError {
    NameEmpty,
    NameTooLong,
    AgeTooYoung,
    AgeTooOld,
    AgeNotANumber,
}

// Name must be non-empty and ≤50 chars. Return Err(NameEmpty) or Err(NameTooLong).
fn validate_name(name: &str) -> Result<&str, ValidationError> {
    todo!()
}

// Parse age string; must be 18–120. Return Err(AgeNotANumber), Err(AgeTooYoung), or Err(AgeTooOld).
fn validate_age(age_str: &str) -> Result<u32, ValidationError> {
    todo!()
}

// Call validate_name and validate_age with ?. Return Ok("Name: Alice, Age: 30") on success.
fn create_profile(name: &str, age_str: &str) -> Result<String, ValidationError> {
    todo!()
}

fn main() {
    println!("{:?}", create_profile("Alice", "30"));  // Ok("Name: Alice, Age: 30")
    println!("{:?}", create_profile("", "30"));        // Err(NameEmpty)
    println!("{:?}", create_profile("Alice", "abc")); // Err(AgeNotANumber)
    println!("{:?}", create_profile("Alice", "15"));  // Err(AgeTooYoung)
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
