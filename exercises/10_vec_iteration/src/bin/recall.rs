// Read ../../README.md before starting.
// Use iterator methods only — no index loops.

// Sum all elements.
fn sum(v: &[i32]) -> i32 {
    todo!()
}
// Return a reference to the largest element, or None if empty.
fn max(v: &[i32]) -> Option<&i32> {
    todo!()
}
// Return a reference to the smallest element, or None if empty.
fn min(v: &[i32]) -> Option<&i32> {
    todo!()
}
// Collect only the even elements into a new Vec.
fn evens(v: &[i32]) -> Vec<i32> {
    todo!()
}
// Collect each element squared into a new Vec.
fn squares(v: &[i32]) -> Vec<i32> {
    todo!()
}
// Count how many elements are strictly greater than 4.
fn count_over_four(v: &[i32]) -> usize {
    todo!()
}

fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("{}", sum(&v));             // 31
    println!("{:?}", max(&v));           // Some(9)
    println!("{:?}", min(&v));           // Some(1)
    println!("{:?}", evens(&v));         // [4, 2, 6]
    println!("{:?}", squares(&v));       // [9, 1, 16, 1, 25, 81, 4, 36]
    println!("{}", count_over_four(&v)); // 3
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
