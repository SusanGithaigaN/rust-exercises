// Read ../../README.md before starting.

// Return the first even number in the slice, or None if there are none.
fn first_even(v: &[i32]) -> Option<i32> {
    todo!()
}

// Return Some(a/b), or None if b is zero.
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

// Return the first element of the slice, or None if empty.
fn head(v: &[i32]) -> Option<i32> {
    todo!()
}

fn main() {
    println!("{:?}", first_even(&[1, 3, 4, 7])); // Some(4)
    println!("{:?}", first_even(&[1, 3, 5, 7])); // None
    println!("{:?}", safe_divide(10.0, 2.0));     // Some(5.0)
    println!("{:?}", safe_divide(10.0, 0.0));     // None
    println!("{:?}", head(&[9, 2, 3]));           // Some(9)
    println!("{:?}", head(&[]));                  // None
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
