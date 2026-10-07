// Read ../../README.md before starting.

fn main() {
    // Write your code here.
    let a: i32 = 5;

    // A block is a collection of statements enclosed by braces {}
    // shadowing: redeclaring a variable using the let keyword again
    {
        let a: i32 = a + 1;
        println!("{a}")
    }

    let spaces: &str = "   ";
    // shadow

    {
        let spaces = spaces.len();
        println!("{spaces}");
    }
}
