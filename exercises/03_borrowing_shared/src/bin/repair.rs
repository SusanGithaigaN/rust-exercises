// Read ../../README.md before starting.

fn get_length(s: &String) -> usize {
    s.len()
}

fn print_twice(s: &String) {
    println!("{} {}", s, s);
}

fn main() {
    // ERROR 1
    let s1 = String::from("hello");
    let len = get_length(&s1);
    println!("len: {}, s1: {}", len, s1);

    // ERROR 2 — uncomment and fix the dangling reference
    // fn make_greeting(name: &str) -> &str {
    //     let greeting = format!("Hello, {}!", name);
    //     &greeting
    // }
    // println!("{}", make_greeting("Alice"));

    // ERROR 3
    let s2 = String::from("world");
    print_twice(&s2);
    println!("s2 still here: {}", s2);
}
