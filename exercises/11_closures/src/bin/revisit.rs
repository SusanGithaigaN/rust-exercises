// Read ../../README.md before starting.

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

fn pipeline(v: &[i32], ops: &[&dyn Fn(i32) -> i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    let add5 = make_adder(5);
    let mul3 = make_multiplier(3);
    println!("{}", add5(10));
    println!("{}", mul3(4));

    let ops: Vec<&dyn Fn(i32) -> i32> = vec![&add5, &mul3];
    println!("{:?}", pipeline(&[1, 2, 3, 4, 5], &ops));
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
