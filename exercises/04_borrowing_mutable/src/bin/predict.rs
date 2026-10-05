// Read ../../README.md before starting.

fn example_a() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    println!("{}", r1);

    let r2 = &mut s;
    r2.push_str("!");
    println!("{}", r2);
}

fn example_b() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);

    let r3 = &mut s;
    r3.push_str("!");
    println!("{}", r3);
}

fn main() {
    example_a();
    example_b();
}
