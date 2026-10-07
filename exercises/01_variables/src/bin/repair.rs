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
    let _x = 10;
    let x = 20;
    println!("x: {}", x);

    // ERROR 2
    let y: f64 = 3.14;
    println!("y: {}", y);

    // ERROR 3
    // let result;
    let result = add(3, 4);
    println!("result: {}", result);
    

    // ERROR 4
    println!("sum: {}", add(1, 2));

    // ERROR 5
    let label:&str = describe(5);
    println!("label: {}", label);
}
