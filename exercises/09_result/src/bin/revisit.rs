// Read ../../README.md before starting.

#[derive(Debug)]
enum ValidationError {
    NameEmpty,
    NameTooLong,
    AgeTooYoung,
    AgeTooOld,
    AgeNotANumber,
}

fn validate_name(name: &str) -> Result<&str, ValidationError> {
    todo!()
}

fn validate_age(age_str: &str) -> Result<u32, ValidationError> {
    todo!()
}

fn create_profile(name: &str, age_str: &str) -> Result<String, ValidationError> {
    todo!()
}

fn main() {
    println!("{:?}", create_profile("Alice", "30"));
    println!("{:?}", create_profile("", "30"));
    println!("{:?}", create_profile("Alice", "abc"));
    println!("{:?}", create_profile("Alice", "15"));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
