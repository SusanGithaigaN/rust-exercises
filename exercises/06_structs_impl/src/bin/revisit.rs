// Read ../../README.md before starting.

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, initial_balance: f64) -> BankAccount {
        todo!()
    }

    fn deposit(&mut self, amount: f64) {
        todo!()
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        todo!()
    }

    fn balance(&self) -> f64 {
        todo!()
    }

    fn statement(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut acc = BankAccount::new("Alice", 100.0);
    println!("{}", acc.statement());
    acc.deposit(50.0);
    println!("{}", acc.statement());
    println!("{}", acc.withdraw(200.0));
    println!("{}", acc.withdraw(80.0));
    println!("{}", acc.statement());
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
