// Read ../../README.md before starting.

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    todo!()
}
fn apply_to_each(v: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| todo!()
}

fn main() {
    println!("{}", apply_twice(|x| x + 10, 3));
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], |x| x * x));

    let double_then_add1 = compose(|x| x + 1, |x| x * 2);
    println!("{:?}", apply_to_each(&[1, 2, 3, 4, 5], double_then_add1));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
