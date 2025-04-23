fn main() {
    let mut account: BankAccount = BankAccount {
        balance: 150.55,
        owner: "Alice".to_string(),
    };

    //immutable borrow to check balance
    account.check_balance();

        //mutable borrow
    account.withdraw(10.1);

    account.check_balance();

}

struct BankAccount {
    balance: f64,
    owner: String,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("WIthdrawing {} from {} 's account", amount, self.owner);
        
        self.balance -= amount;
        
    }
    fn check_balance(&self) {
        println!("{}'s account balance is {}", self.owner, self.balance)
    }
}
