// TOPIC 13 — Modules and Visibility
// EXERCISE TYPE: Recall
// =============================================
// Organise code into modules using inline `mod` blocks.
//
// Build this structure inside this single file:
//
//   mod geometry {
//       pub struct Point { pub x: f64, pub y: f64 }
//       impl Point {
//           pub fn new(x: f64, y: f64) -> Point
//           pub fn distance(&self, other: &Point) -> f64
//             Hint: ((self.x-other.x).powi(2) + (self.y-other.y).powi(2)).sqrt()
//       }
//
//       pub mod shapes {
//           use super::Point;
//           pub struct Circle { pub center: Point, pub radius: f64 }
//           impl Circle {
//               pub fn new(center: Point, radius: f64) -> Circle
//               pub fn area(&self) -> f64         // π * r²
//               pub fn contains(&self, p: &Point) -> bool
//                 Hint: distance from center to p < radius
//           }
//       }
//   }
//
// In main:
//   - use geometry::Point and geometry::shapes::Circle
//   - create two Points, print their distance
//   - create a Circle, print its area
//   - check if a point is inside the circle
//
// Expected output:
//   distance: 5
//   circle area: 78.53981633974483
//   (3,4) inside circle: true
//   (10,10) inside circle: false
//
// Run: cargo run --bin recall

fn main() {
    // TODO: define the modules above, then use them here
}
