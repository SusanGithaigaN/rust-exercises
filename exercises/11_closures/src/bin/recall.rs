// Read ../../README.md before starting.

// Call f twice on x: apply_twice(f, 3) where f adds 10 → f(f(3)) = 23.
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    todo!()
}

// Apply f to every element and collect the results into a new Vec.
fn apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{}", apply_twice(|x| x + 10, 3));               // 23
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x * x)); // [1, 4, 9, 16, 25]
    let n = 5;
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x + n)); // [6, 7, 8, 9, 10]
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
