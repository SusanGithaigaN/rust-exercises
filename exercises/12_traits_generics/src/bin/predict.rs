// TOPIC 12 — Traits and Generics
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, answer these in your notes:
//   1. What will largest(&numbers) return?
//   2. What will largest(&chars) return?
//   3. Why does print_pair(3, "hello") not compile?
//   4. What does the T: PartialOrd bound mean?
//
// Run: cargo run --bin predict

use std::fmt::Display;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_pair<T: Display>(a: T, b: T) {
    println!("({}, {})", a, b);
}

trait Greet {
    fn greeting(&self) -> String;

    fn greet(&self) {
        println!("{}", self.greeting());
    }
}

struct English;
struct Spanish;

impl Greet for English {
    fn greeting(&self) -> String {
        String::from("Hello!")
    }
}

impl Greet for Spanish {
    fn greeting(&self) -> String {
        String::from("Hola!")
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));

    print_pair(3, 7);
    print_pair("hello", "world");
    // print_pair(3, "hello"); // uncomment — why does this fail?

    let e = English;
    let s = Spanish;
    e.greet();
    s.greet();
    println!("{}", e.greeting());
}
