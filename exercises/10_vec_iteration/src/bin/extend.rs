// Read ../../README.md before starting.
// Use iterator methods only — no index loops.

fn sum(v: &[i32]) -> i32 { todo!() }
fn max(v: &[i32]) -> Option<&i32> { todo!() }
fn min(v: &[i32]) -> Option<&i32> { todo!() }
fn evens(v: &[i32]) -> Vec<i32> { todo!() }
fn squares(v: &[i32]) -> Vec<i32> { todo!() }
fn count_over_four(v: &[i32]) -> usize { todo!() }

// Return running totals: [1,2,3] → [1,3,6].
fn running_totals(v: &[i32]) -> Vec<i32> {
    todo!()
}

// Flatten a slice of Vecs into one Vec: [[1,2],[3],[4,5,6]] → [1,2,3,4,5,6].
fn flatten_vecs(v: &[Vec<i32>]) -> Vec<i32> {
    todo!()
}

fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("{}", sum(&v));                              // 31
    println!("{:?}", evens(&v));                          // [4, 2, 6]
    println!("{:?}", running_totals(&[1, 2, 3, 4, 5]));  // [1, 3, 6, 10, 15]
    println!("{:?}", running_totals(&[5, 5, 5]));         // [5, 10, 15]
    println!("{:?}", flatten_vecs(&[vec![1, 2], vec![3], vec![4, 5, 6]])); // [1,2,3,4,5,6]
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
