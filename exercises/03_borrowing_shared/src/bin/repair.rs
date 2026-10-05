// TOPIC 03 — Borrowing: Shared References
// EXERCISE TYPE: Repair
// =============================================
// Fix the borrowing errors. Do not change the function signatures.
// Do not add .clone() — fix the borrow instead.
//
// Run: cargo run --bin repair

fn get_length(s: &String) -> usize {
    s.len()
}

fn print_twice(s: &String) {
    println!("{} {}", s, s);
}

fn main() {
    // ERROR 1: get_length expects a reference but is given an owned String by move
    let s1 = String::from("hello");
    let len = get_length(s1);
    println!("len: {}, s1: {}", len, s1);

    // ERROR 2: the return value of this function dangles (references local data)
    // Uncomment the function below and fix it
    // fn make_greeting(name: &str) -> &str {
    //     let greeting = format!("Hello, {}!", name);
    //     &greeting  // returning reference to local variable — fix this
    // }
    // println!("{}", make_greeting("Alice"));

    // ERROR 3: print_twice is called with an owned value, not a reference
    let s2 = String::from("world");
    print_twice(s2);
    println!("s2 still here: {}", s2);
}
