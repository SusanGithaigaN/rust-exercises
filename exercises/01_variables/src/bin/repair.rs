// Read ../../README.md before starting.

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
    // ERROR 1
    let x = 10;
    x = 20;
    println!("x: {}", x);

    // ERROR 2
    let y: i32 = 3.14;
    println!("y: {}", y);

    // ERROR 3
    let result;
    println!("result: {}", result);
    result = add(3, 4);

    // ERROR 4
    println!("sum: {}", add(1, 2, 3));

    // ERROR 5
    let label: i32 = describe(5);
    println!("label: {}", label);
}
