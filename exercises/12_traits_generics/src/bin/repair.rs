// Read ../../README.md before starting.

use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}
struct Tweet {
    username: String,
    body: String,
}

// ERROR 1
impl Summary for Article {
    fn summarize_content(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..20])
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.body)
    }
}

// ERROR 2
fn print_twice<T>(item: T) {
    println!("{}", item);
    println!("{}", item);
}

// ERROR 3
fn largest<T>(list: &[T]) -> T {
    let mut l = list[0];
    for &item in list {
        if item > l {
            l = item;
        }
    }
    l
}

fn main() {
    let a = Article {
        title: String::from("Rust"),
        content: String::from("Rust is a systems language..."),
    };
    println!("{}", a.summarize());
    print_twice(42);
    let nums = vec![3, 1, 4, 1, 5];
    println!("largest: {}", largest(&nums));
}
