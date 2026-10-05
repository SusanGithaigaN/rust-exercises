// TOPIC 02 — Ownership: Moves and Scope
// EXERCISE TYPE: Repair
// =============================================
// Fix all ownership errors. Do not add .clone() unless a comment says you may.
// Read each error message carefully — they explain exactly what moved where.
//
// Run: cargo run --bin repair

fn print_string(s: String) {
    println!("{}", s);
}

fn string_length(s: String) -> usize {
    s.len()
}

fn main() {
    // ERROR 1: s is moved into print_string, then used again
    let s = String::from("hello");
    print_string(s);
    println!("s is: {}", s);

    // ERROR 2: s2 is moved to s3, then s2 is used
    let s2 = String::from("world");
    let s3 = s2;
    println!("{} {}", s2, s3);

    // ERROR 3: s4 is moved into string_length, then printed
    // You MAY use .clone() here — it's the intended fix for this one
    let s4 = String::from("rust");
    let len = string_length(s4);
    println!("s4: {}, len: {}", s4, len);

    // ERROR 4: an integer is treated like it moves (it shouldn't — why not?)
    // Fix by understanding, not by changing the code:
    // Uncomment this block and explain in a comment why it works:
    // let n = 42;
    // let m = n;
    // println!("{} {}", n, m);
}
