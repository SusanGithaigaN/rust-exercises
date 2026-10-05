// TOPIC 06 — Structs and impl
// EXERCISE TYPE: Recall
// =============================================
// Define a Rectangle struct and implement methods on it.
//
// struct Rectangle { width: f64, height: f64 }
//
// impl Rectangle {
//   fn new(width: f64, height: f64) -> Rectangle   <- associated function (no &self)
//   fn area(&self) -> f64                           <- width * height
//   fn perimeter(&self) -> f64                      <- 2 * (width + height)
//   fn is_square(&self) -> bool                     <- width == height
//   fn scale(&self, factor: f64) -> Rectangle       <- returns a NEW Rectangle, does not mutate
// }
//
// Also add #[derive(Debug)] above the struct definition so you can print with {:?}
//
// Expected output:
//   rect: Rectangle { width: 4.0, height: 6.0 }
//   area: 24
//   perimeter: 20
//   is square: false
//   scaled by 2: Rectangle { width: 8.0, height: 12.0 }
//   square: Rectangle { width: 5.0, height: 5.0 }
//   is square: true
//
// Run: cargo run --bin recall

fn main() {
    let rect = Rectangle::new(4.0, 6.0);
    // TODO: implement Rectangle and its methods, then fill in the println! calls here
    println!("rect: {:?}", rect);
    println!("area: {}", rect.area());
    println!("perimeter: {}", rect.perimeter());
    println!("is square: {}", rect.is_square());
    println!("scaled by 2: {:?}", rect.scale(2.0));

    let square = Rectangle::new(5.0, 5.0);
    println!("square: {:?}", square);
    println!("is square: {}", square.is_square());
}
