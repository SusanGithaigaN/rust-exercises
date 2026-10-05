// TOPIC 07 — Enums and match
// EXERCISE TYPE: Recall
// =============================================
// Define two enums and write functions that use match on them.
//
// Part A:
//   enum Direction { North, South, East, West }
//   fn opposite(d: Direction) -> Direction    <- North<->South, East<->West
//   fn is_horizontal(d: Direction) -> bool    <- East or West
//   fn describe(d: Direction) -> &'static str <- "heading north", "heading south", etc.
//
// Part B:
//   enum Coin { Penny, Nickel, Dime, Quarter }
//   fn value_in_cents(coin: Coin) -> u32      <- 1, 5, 10, 25
//
// Note: match must be exhaustive — handle every variant.
//
// Expected output:
//   opposite of North: South
//   opposite of East: West
//   East is horizontal: true
//   North is horizontal: false
//   describing South: heading south
//   Quarter = 25 cents
//   Penny = 1 cent
//
// Run: cargo run --bin recall

fn main() {
    // TODO: define the enums and functions above, then print the expected output
}
