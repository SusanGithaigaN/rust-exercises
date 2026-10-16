// Read ../../README.md before starting.
use capitalize::Capitalize;

fn append_greeting(s: &mut String) {
    // todo!()
    s.push_str(", hello!");
}
fn double_all(v: &mut Vec<i32>) {
    // todo!()
    // .iter_mut() creates a mutable iterator over a collection,
    // allowing you to loop through its elements and modify each item in place.
    for i in v.iter_mut() {
        *i *= 2;
    }
}
fn clamp_to_zero(n: &mut i32) {
    // todo!()
    // let mut neg: = 0;
    // for i in n.len() {
    if *n < 0 {
        *n = 0;
    }
}

fn capitalize_first(s: &mut String) {
    // todo!()
    *s = s.capitalize_first_only();
}

fn main() {
    let mut s = String::from("world");
    append_greeting(&mut s);
    println!("{}", s);

    let mut v = vec![1, 3, 5, 7];
    double_all(&mut v);
    println!("{:?}", v);

    let mut a = String::from("hello");
    capitalize_first(&mut a);
    println!("{}", a);

    let mut b = String::from("rust is great");
    capitalize_first(&mut b);
    println!("{}", b);
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/extend.rs"));
