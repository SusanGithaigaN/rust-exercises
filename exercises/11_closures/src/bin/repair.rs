// Read ../../README.md before starting.

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // ERROR 1
    |x| x + n
}

fn main() {
    // ERROR 2
    let s = String::from("hello");
    let consume = move || {
        println!("{}", s);
        s
    };
    consume();
    consume();

    // ERROR 3
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    inc();
    println!("count: {}", count);
    inc();

    // ERROR 4
    let mut total = 0;
    let accumulate = |x: i32| {
        total += x;
        total
    };
    println!("{}", apply(accumulate, 5));
}
