// Read ../../README.md before starting.

use std::string;

fn append_greeting(s: &mut String) {
    // todo!()
    let new_str: String = ", hello!".to_owned();
    s.push_str(&new_str)
}

fn double_all(v: &mut Vec<i32>) {
    // todo!()
    // v.repeat(2);
    for i in v.iter_mut() {
        *i *= 2;
    }
    return;
}

fn clamp_to_zero(n: &mut i32) {
    // todo!()
    // let div = 2;
    //  dereference operator(*) it tells rust:
    // go to the memory address that n is pointing to(*n),
    // and look the actual integer value stored there
    if *n < 0 {
        *n = 0;
    }
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
