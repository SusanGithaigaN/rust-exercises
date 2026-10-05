// Read ../../README.md before starting.

fn main() {
    let x = 5;
    let add_x = |n| n + x;
    println!("{}", add_x(3));
    println!("{}", add_x(10));
    println!("x still: {}", x);

    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("{}", increment());
    println!("{}", increment());
    println!("{}", increment());
    // println!("{}", count);
    println!("count after drop: {}", count);

    let s = String::from("hello");
    let greet = move || println!("greeting: {}", s);
    greet();
    greet();
    // println!("{}", s);
}
