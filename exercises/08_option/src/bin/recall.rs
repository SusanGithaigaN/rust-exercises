// Read ../../README.md before starting.

fn first_even(v: &[i32]) -> Option<i32> {
    todo!()
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

fn head(v: &[i32]) -> Option<i32> {
    todo!()
}

fn main() {
    println!("{:?}", first_even(&[1, 3, 4, 7]));
    println!("{:?}", first_even(&[1, 3, 5, 7]));
    println!("{:?}", safe_divide(10.0, 2.0));
    println!("{:?}", safe_divide(10.0, 0.0));
    println!("{:?}", head(&[9, 2, 3]));
    println!("{:?}", head(&[]));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
