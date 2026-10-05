// TOPIC 01 — Variables, Mutability, Functions
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running this code, write down what you expect it to print.
// Then run it and compare.
//
// Questions to answer in your notes before running:
//   1. What is the value of x at each println?
//   2. Is this mutating x, or creating new bindings?
//   3. Why can `spaces` change from &str to usize?
//   4. What does the inner scope do to the outer x?
//
// Run: cargo run --bin predict

fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("inner x: {}", x);
    }

    println!("outer x: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let mut count = 0;
    count += 1;
    count += 1;
    count += 1;
    println!("count: {}", count);

    println!("square(4) = {}", square(4));
    println!("square(square(2)) = {}", square(square(2)));
}
