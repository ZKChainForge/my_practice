#[derive(Debug)]
enum BankError {
    InsufficientFunds,
}

struct Account {
    owner: String,
    balance: f64,
}

impl Account {
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(BankError::InsufficientFunds)
        }
    }
}

pub fn run() {
    let mut account = Account {
        owner: "Alice".to_string(),
        balance: 100.0,
    };

    match account.withdraw(150.0) {
        Ok(_) => println!("Withdrawal successful"),
        Err(e) => println!("Error: {:?}", e),
    }
}
