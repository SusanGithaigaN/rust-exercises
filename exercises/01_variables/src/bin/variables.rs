// Read ../../README.md before starting.

fn main() {
    // Write your code here.
    let mut a: i32 = 5;
    a += 3;
    println!("{a}");
    // immutable variable. The value cannot be changed'
    // In Rust, variables are immutable by default
    {
        let name = "Rust";
        println!("{name}")
    }
    // immutable boolean
    {
        let is_learning: bool = true;
        println!("{is_learning}")
    }
}
