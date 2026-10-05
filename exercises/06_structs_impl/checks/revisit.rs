#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new() {
        let a = BankAccount::new("Alice", 100.0);
        assert_eq!(a.owner, "Alice");
        assert_eq!(a.balance, 100.0);
    }

    #[test]
    fn check_deposit() {
        let mut a = BankAccount {
            owner: "Alice".into(),
            balance: 100.0,
        };
        a.deposit(50.0);
        assert_eq!(a.balance, 150.0);
        a.deposit(-10.0);
        a.deposit(0.0);
        assert_eq!(a.balance, 150.0);
    }

    #[test]
    fn check_withdraw() {
        let mut a = BankAccount {
            owner: "Alice".into(),
            balance: 100.0,
        };
        assert!(!a.withdraw(200.0));
        assert!(!a.withdraw(-1.0));
        assert!(!a.withdraw(0.0));
        assert_eq!(a.balance, 100.0);
        assert!(a.withdraw(100.0));
        assert_eq!(a.balance, 0.0);
    }

    #[test]
    fn check_balance() {
        let a = BankAccount {
            owner: "Alice".into(),
            balance: 70.0,
        };
        assert_eq!(a.balance(), 70.0);
        assert_eq!(a.owner, "Alice");
    }

    #[test]
    fn check_statement() {
        let a = BankAccount {
            owner: "Alice".into(),
            balance: 70.0,
        };
        assert_eq!(a.statement(), "Alice: $70.00");
    }
}
