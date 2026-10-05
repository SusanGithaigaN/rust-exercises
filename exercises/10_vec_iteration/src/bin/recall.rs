// Read ../../README.md before starting.

fn sum(v: &[i32]) -> i32 {
    todo!()
}
fn max(v: &[i32]) -> Option<&i32> {
    todo!()
}
fn min(v: &[i32]) -> Option<&i32> {
    todo!()
}
fn evens(v: &[i32]) -> Vec<i32> {
    todo!()
}
fn squares(v: &[i32]) -> Vec<i32> {
    todo!()
}
fn count_over_four(v: &[i32]) -> usize {
    todo!()
}

fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("{}", sum(&v));
    println!("{:?}", max(&v));
    println!("{:?}", min(&v));
    println!("{:?}", evens(&v));
    println!("{:?}", squares(&v));
    println!("{}", count_over_four(&v));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
