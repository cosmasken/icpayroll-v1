use crate::payments::balances;
use crate::payments::subscriptions;
use std::collections::HashMap;

#[derive(Default)]
pub struct PaymentProcessor {
    pub transactions: HashMap<u64, Vec<Transaction>>, // User ID -> List of Transactions
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub amount: u128,
    pub description: String,
    pub timestamp: u64,
    pub status: PaymentStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

pub fn process_payment(user_id: u64, amount: u128, description: String) -> Result<(), String> {
    let mut balances = ic_cdk::storage::get_mut::<balances::BalanceManager>();

    // Ensure the user has sufficient balance
    if !balances.deduct_balance(user_id, amount) {
        return Err("Insufficient balance".to_string());
    }

    let processor = ic_cdk::storage::get_mut::<PaymentProcessor>();

    // Record the transaction
    let transaction = Transaction {
        amount,
        description,
        timestamp: ic_cdk::api::time(),
        status: PaymentStatus::Pending,
    };

    processor
        .transactions
        .entry(user_id)
        .or_default()
        .push(transaction);

    Ok(())
}
