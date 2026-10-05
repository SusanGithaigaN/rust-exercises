// Read ../../README.md before starting.

fn print_it(s: &str) {
    println!("got: {}", s);
}

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let whole: &str = &s[..];
    println!("whole: {}", whole);

    let literal: &str = "this is a literal";
    let first = &literal[0..4];
    println!("first 4: {}", first);

    print_it(&s);
    print_it(literal);
    print_it(hello);
    print_it("direct");
}
