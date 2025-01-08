use std::collections::HashMap;

#[derive(Default)]
pub struct BalanceManager {
    pub balances: HashMap<u64, u128>, // User ID -> Balance
}

impl BalanceManager {
    pub fn get_balance(&self, user_id: u64) -> u128 {
        *self.balances.get(&user_id).unwrap_or(&0)
    }

    pub fn add_balance(&mut self, user_id: u64, amount: u128) {
        *self.balances.entry(user_id).or_insert(0) += amount;
    }

    pub fn deduct_balance(&mut self, user_id: u64, amount: u128) -> bool {
        if let Some(balance) = self.balances.get_mut(&user_id) {
            if *balance >= amount {
                *balance -= amount;
                return true;
            }
        }
        false
    }
}
