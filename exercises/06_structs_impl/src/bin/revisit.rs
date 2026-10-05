// TOPIC 06 — Structs and impl
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Build a BankAccount struct.
//
// #[derive(Debug)]
// struct BankAccount { owner: String, balance: f64 }
//
// impl BankAccount {
//   fn new(owner: &str, initial_balance: f64) -> BankAccount
//   fn deposit(&mut self, amount: f64)          <- adds amount, no return
//   fn withdraw(&mut self, amount: f64) -> bool <- returns false if insufficient funds
//   fn balance(&self) -> f64
//   fn statement(&self) -> String               <- "{owner}: ${balance:.2}"
// }
//
// Expected output:
//   Alice: $100.00
//   after deposit 50: Alice: $150.00
//   withdrew 200: false
//   withdrew 80: true
//   Alice: $70.00
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement BankAccount above main, then use it here
}
