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

fn find_index(v: &[i32], target: i32) -> Option<usize> {
    todo!()
}

fn last_element(v: &[i32]) -> Option<i32> {
    todo!()
}

fn main() {
    println!("{:?}", first_even(&[1, 3, 4, 7]));
    println!("{:?}", safe_divide(10.0, 0.0));
    println!("{:?}", head(&[]));
    println!("{:?}", find_index(&[1, 3, 5, 7, 9], 5));
    println!("{:?}", find_index(&[1, 3, 5, 7, 9], 4));
    println!("{:?}", last_element(&[9, 2, 3]));
    println!("{:?}", last_element(&[]));

    let v = vec![1, 3, 5, 7, 9];
    let result = find_index(&v, 5).and_then(|i| v.get(i)).map(|&x| x * x);
    println!("{:?}", result);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
