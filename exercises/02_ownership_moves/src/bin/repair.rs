// Read ../../README.md before starting.

fn print_string(s: String) {
    println!("{}", s);
}

fn string_length(s: String) -> usize {
    s.len()
}

fn main() {
    // ERROR 1
    let s = String::from("hello");
    print_string(s);
    println!("s is: {}", s);

    // ERROR 2
    let s2 = String::from("world");
    let s3 = s2;
    println!("{} {}", s2, s3);

    // ERROR 3 — .clone() is the intended fix here
    let s4 = String::from("rust");
    let len = string_length(s4);
    println!("s4: {}, len: {}", s4, len);
}
