// TOPIC 05 — Slices and Strings
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. What type is `hello`? What type is `world`?
//   2. What does &s[..] give you?
//   3. Why does print_it accept both &String and &str?
//   4. What is the type of `first` in this example?
//
// Run: cargo run --bin predict

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

    // print_it works with all three — why?
    print_it(&s);       // &String
    print_it(literal);  // &str
    print_it(hello);    // slice
    print_it("direct"); // literal
}
