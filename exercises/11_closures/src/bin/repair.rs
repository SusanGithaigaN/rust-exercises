// TOPIC 11 — Closures
// EXERCISE TYPE: Repair
// =============================================
// Fix the closure errors.
//
// Run: cargo run --bin repair

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // ERROR 1: returning a closure that references a local variable
    // n is a parameter — but the closure captures it by reference.
    // Fix: use the `move` keyword.
    |x| x + n
}

fn main() {
    // ERROR 2: calling an FnOnce closure twice
    let s = String::from("hello");
    let consume = move || {
        println!("{}", s);
        s // moves s out — this is FnOnce
    };
    consume();
    consume(); // fix: only call it once, or don't move s out

    // ERROR 3: `count` is borrowed by the closure and also used directly
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    inc();
    println!("count: {}", count); // fix: drop inc before using count, or restructure
    inc();

    // ERROR 4: closure type mismatch — apply expects Fn(i32)->i32 but gets FnMut
    let mut total = 0;
    let accumulate = |x: i32| {
        total += x;
        total
    };
    println!("{}", apply(accumulate, 5));
}
