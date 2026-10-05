// Read ../../README.md before starting.

fn print_len(s: &String) {
    println!("length: {}", s.len());
}

fn first_byte(s: &String) -> u8 {
    s.as_bytes()[0]
}

fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    print_len(&s);
    print_len(&s);

    let b = first_byte(&s);
    println!("first byte: {}", b);

    println!("s is still here: {}", s);

    let r3 = &s;
    let r4 = &s;
    let r5 = &s;
    println!("{} {} {}", r3, r4, r5);
    println!("s after many borrows: {}", s);
}
