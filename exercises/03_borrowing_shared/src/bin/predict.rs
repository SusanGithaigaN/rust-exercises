// TOPIC 03 — Borrowing: Shared References
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. How many shared references can exist at the same time?
//   2. Does borrowing (shared) transfer ownership?
//   3. After all the borrows, is s still valid?
//   4. Why can print_len be called multiple times?
//
// Run: cargo run --bin predict

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

    // Multiple borrows at the same scope:
    let r3 = &s;
    let r4 = &s;
    let r5 = &s;
    println!("{} {} {}", r3, r4, r5);
    println!("s after many borrows: {}", s);
}
