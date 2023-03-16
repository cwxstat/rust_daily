pub trait Transaction {
    fn execute(&self, balance: &mut f64);
}

pub struct Deposit {
    pub amount: f64,
}

impl Transaction for Deposit {
    fn execute(&self, balance: &mut f64) {
        *balance += self.amount;
        println!("Deposit: ${}", self.amount);
    }
}

pub struct Withdraw {
    pub amount: f64,
}

impl Transaction for Withdraw {
    fn execute(&self, balance: &mut f64) {
        if *balance >= self.amount {
            *balance -= self.amount;
            println!("Withdraw: ${}", self.amount);
        } else {
            println!("Withdrawal failed: Insufficient funds");
        }
    }
}

pub struct Bank {
    pub transactions: Vec<Box<dyn Transaction>>,
    pub balance: f64,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            balance: 0.0,
        }
    }

    pub fn add_transaction<T: Transaction + 'static>(&mut self, transaction: T) {
        self.transactions.push(Box::new(transaction));
    }

    pub fn process_transactions(&mut self) {
        for transaction in &self.transactions {
            transaction.execute(&mut self.balance);
        }
    }
}

fn main() {
    let deposit = Deposit { amount: 1000.0 };
    let withdraw = Withdraw { amount: 500.0 };
    let invalid_withdraw = Withdraw { amount: 2000.0 };

    let mut bank = Bank::new();
    bank.add_transaction(deposit);
    bank.add_transaction(withdraw);
    bank.add_transaction(invalid_withdraw);

    bank.process_transactions();
}
