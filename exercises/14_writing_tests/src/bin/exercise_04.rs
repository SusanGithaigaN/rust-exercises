// Read ../../README.md before starting.

fn get_element(v: &[i32], index: usize) -> i32 {
    if index >= v.len() {
        panic!("index {} out of bounds for length {}", index, v.len());
    }
    v[index]
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// Write your tests here.

fn main() {
    let v = vec![10, 20, 30];
    println!("element at 1: {}", get_element(&v, 1));
    println!("10 / 2: {:?}", safe_divide(10, 2));
    println!("10 / 0: {:?}", safe_divide(10, 0));
    println!("Run 'cargo test --bin exercise_04' to run your tests.");
}
