// Read ../../README.md before starting.

fn double(n: i32) -> i32 {
    n * 2
}

fn clamp(n: i32, min: i32, max: i32) -> i32 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Write your tests here.

fn main() {
    println!("double(5) = {}", double(5));
    println!("clamp(15, 0, 10) = {}", clamp(15, 0, 10));
    println!("reverse \"hello\" = {}", reverse_string("hello"));
    println!("Run 'cargo test --bin exercise_01' to run your tests.");
}
