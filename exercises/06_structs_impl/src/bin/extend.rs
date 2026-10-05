// TOPIC 06 — Structs and impl
// EXERCISE TYPE: Extend
// =============================================
// Rectangle from recall is provided. Add one method and one new struct.
//
// ADD to Rectangle:
// fn can_contain(&self, other: &Rectangle) -> bool
//   Returns true if self can fully contain other (both width and height are larger).
//
// ADD a new struct:
// #[derive(Debug)]
// struct Circle { radius: f64 }
// impl Circle {
//   fn new(radius: f64) -> Circle
//   fn area(&self) -> f64        // π * r²  (use std::f64::consts::PI)
//   fn circumference(&self) -> f64  // 2 * π * r
// }
//
// Expected output:
//   10x8 can contain 5x3: true
//   5x3 can contain 10x8: false
//   circle r=3 area: 28.274333882308138
//   circle r=3 circumference: 18.84955592153876
//
// Run: cargo run --bin extend

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle { Rectangle { width, height } }
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn is_square(&self) -> bool { self.width == self.height }
    fn scale(&self, factor: f64) -> Rectangle { Rectangle::new(self.width * factor, self.height * factor) }
    // TODO: add can_contain here
}

// TODO: add Circle struct and impl here

fn main() {
    let big = Rectangle::new(10.0, 8.0);
    let small = Rectangle::new(5.0, 3.0);
    // TODO: test can_contain and Circle here
    println!("big area: {}", big.area());
    println!("small perimeter: {}", small.perimeter());
}
