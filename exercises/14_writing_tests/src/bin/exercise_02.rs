// Read ../../README.md before starting.

fn find_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

fn parse_positive(s: &str) -> Result<u32, String> {
    if s.is_empty() {
        return Err(String::from("input is empty"));
    }
    match s.parse::<u32>() {
        Ok(0) => Err(String::from("number must be positive")),
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{}' is not a valid positive number", s)),
    }
}

// Write your tests here.

fn main() {
    println!("find_even([1,3,4]) = {:?}", find_even(&[1, 3, 4]));
    println!("parse_positive(\"5\") = {:?}", parse_positive("5"));
    println!("Run 'cargo test --bin exercise_02' to run your tests.");
}
