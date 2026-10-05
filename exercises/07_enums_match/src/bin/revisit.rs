// TOPIC 07 — Enums and match
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Represent geometric shapes with an enum and implement calculations.
//
// enum Shape {
//   Circle(f64),              // radius
//   Rectangle(f64, f64),      // width, height
//   Triangle(f64, f64, f64),  // three sides
// }
//
// fn area(shape: &Shape) -> f64
//   Circle:    π * r²
//   Rectangle: width * height
//   Triangle:  Heron's formula: let s=(a+b+c)/2; (s*(s-a)*(s-b)*(s-c)).sqrt()
//
// fn describe(shape: &Shape) -> String
//   "Circle with radius 3"
//   "Rectangle 4 x 6"
//   "Triangle with sides 3, 4, 5"
//
// fn is_valid(shape: &Shape) -> bool
//   Circle: radius > 0
//   Rectangle: both sides > 0
//   Triangle: sum of any two sides must be > third side
//
// Expected output:
//   Circle with radius 3: area = 28.27...
//   Rectangle 4 x 6: area = 24
//   Triangle 3,4,5: area = 6, valid: true
//   Triangle 1,2,10: valid: false
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement Shape, area, describe, and is_valid above main, then use here
}
