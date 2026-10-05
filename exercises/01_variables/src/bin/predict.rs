// Read ../../README.md before starting.

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
