// TOPIC 01 — Variables, Mutability, Functions
// EXERCISE TYPE: Repair
// =============================================
// This code has intentional errors. Fix all of them.
// Do NOT add new functionality — only fix what's broken.
//
// Strategy: read the full compiler error before touching any code.
// Fix one error at a time and re-run after each fix.
//
// Run: cargo run --bin repair

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn describe(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else {
        "not positive"
    }
}

fn main() {
    // ERROR 1: x needs to be mutable but isn't declared as such
    let x = 10;
    x = 20;
    println!("x: {}", x);

    // ERROR 2: wrong type — 3.14 cannot go into an i32 variable
    let y: i32 = 3.14;
    println!("y: {}", y);

    // ERROR 3: result is used before it has a value
    let result;
    println!("result: {}", result);
    result = add(3, 4);

    // ERROR 4: this function is called with the wrong number of arguments
    println!("sum: {}", add(1, 2, 3));

    // ERROR 5: wrong return type annotation — the function returns &str
    let label: i32 = describe(5);
    println!("label: {}", label);
}
