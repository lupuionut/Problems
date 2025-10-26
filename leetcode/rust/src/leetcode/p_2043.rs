// 2043. Simple Bank System
// ------------------------
struct Bank {
    accounts: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { accounts: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let a1 = account1 as usize - 1;
        let a2 = account2 as usize - 1;
        if a1 >= self.accounts.len() || a2 >= self.accounts.len() {
            return false;
        }
        if self.accounts[a1] < money {
            return false;
        }
        self.accounts[a1] -= money;
        self.accounts[a2] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize - 1;
        if a >= self.accounts.len() {
            return false;
        }
        self.accounts[a] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize - 1;
        if a >= self.accounts.len() {
            return false;
        }
        if self.accounts[a] < money {
            return false;
        }
        self.accounts[a] -= money;
        true
    }
}
