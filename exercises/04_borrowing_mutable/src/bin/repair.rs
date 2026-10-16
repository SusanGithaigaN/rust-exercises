// Read ../../README.md before starting.

fn main() {
    // ERROR 1
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);

    // ERROR 2
    let mut v = vec![1, 2, 3];
    v.push(4);
    let first = v[0];
    println!("first: {}", first);

    // ERROR 3
    let mut n = 10;
    let r: &mut i32 = &mut n;
    *r = 20;
    println!("{}", n);
}
