// Read ../../README.md before starting.

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    todo!()
}

fn apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{}", apply_twice(|x| x + 10, 3));
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x * x));
    let n = 5;
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x + n));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
