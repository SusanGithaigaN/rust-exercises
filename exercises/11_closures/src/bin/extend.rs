// Read ../../README.md before starting.

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    todo!()
}
fn apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

// Return a closure that applies g first, then f: compose(f,g)(x) == f(g(x)).
fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| todo!()
}

fn main() {
    println!("{}", apply_twice(|x| x + 10, 3));               // 23
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x * x)); // [1, 4, 9, 16, 25]

    let double_then_add1 = compose(|x| x + 1, |x| x * 2);
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], double_then_add1)); // [3, 5, 7, 9, 11]
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
