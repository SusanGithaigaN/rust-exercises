// Read ../../README.md before starting.

fn append_greeting(s: &mut String) {
    todo!()
}

fn double_all(v: &mut Vec<i32>) {
    todo!()
}

fn clamp_to_zero(n: &mut i32) {
    todo!()
}

fn main() {
    let mut s = String::from("world");
    append_greeting(&mut s);
    println!("{}", s);

    let mut v = vec![1, 3, 5, 7];
    double_all(&mut v);
    println!("{:?}", v);

    let mut a = -5;
    clamp_to_zero(&mut a);
    println!("{}", a);

    let mut b = 3;
    clamp_to_zero(&mut b);
    println!("{}", b);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
