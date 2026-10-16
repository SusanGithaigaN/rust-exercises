// Read ../../README.md before starting.

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Create an account with the given owner name and starting balance.
    fn new(owner: &str, initial_balance: f64) -> BankAccount {
        todo!()
    }

    // Add amount to the balance.
    fn deposit(&mut self, amount: f64) {
        todo!()
    }

    // Subtract amount if funds are available. Return true on success, false if insufficient.
    fn withdraw(&mut self, amount: f64) -> bool {
        todo!()
    }

    // Return the current balance.
    fn balance(&self) -> f64 {
        todo!()
    }

    // Return a string like "Alice: $70.00".
    fn statement(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut acc = BankAccount::new("Alice", 100.0);
    println!("{}", acc.statement());     // Alice: $100.00
    acc.deposit(50.0);
    println!("{}", acc.statement());     // Alice: $150.00
    println!("{}", acc.withdraw(200.0)); // false
    println!("{}", acc.withdraw(80.0));  // true
    println!("{}", acc.statement());     // Alice: $70.00
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
