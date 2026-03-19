fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow
    account.check_balance();

    // Mutable borrow
    account.withdraw(45.5);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has the balance of {}",
            self.owner, self.balance
        );
    }
}
