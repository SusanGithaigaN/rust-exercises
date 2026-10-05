// TOPIC 11 — Closures
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. What does add_x(3) print? What does add_x(10) print?
//   2. Is x moved into the closure or borrowed?
//   3. What does increment() return each time it's called?
//   4. Why can't you use `count` while `increment` is alive?
//   5. The commented-out line — would it compile? Why?
//
// Run: cargo run --bin predict

fn main() {
    let x = 5;
    let add_x = |n| n + x;
    println!("{}", add_x(3));
    println!("{}", add_x(10));
    println!("x still: {}", x); // x was borrowed, not moved

    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("{}", increment());
    println!("{}", increment());
    println!("{}", increment());
    // println!("{}", count); // uncomment — does this compile here?
    drop(increment);
    println!("count after drop: {}", count);

    // move closure
    let s = String::from("hello");
    let greet = move || println!("greeting: {}", s);
    greet();
    greet(); // can we call it twice?
    // println!("{}", s); // uncomment — does this compile?
}
