// Read ../../README.md before starting.

// Return a closure that adds n to its argument. Capture n by move.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

// Return a closure that multiplies its argument by n. Capture n by move.
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| todo!()
}

// Apply every closure in ops to each element; return the final Vec.
fn pipeline(v: &[i32], ops: &[&dyn Fn(i32) -> i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    let add5 = make_adder(5);
    let mul3 = make_multiplier(3);
    println!("{}", add5(10)); // 15
    println!("{}", mul3(4));  // 12

    let ops: Vec<&dyn Fn(i32) -> i32> = vec![&add5, &mul3];
    println!("{:?}", pipeline(&[1, 2, 3, 4, 5], &ops)); // [18, 21, 24, 27, 30]
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
