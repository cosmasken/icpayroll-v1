// src/reports/transaction_reports.rs

pub fn generate_transaction_report(transaction_id: &str, amount: u128, status: &str) -> String {
    format!(
        "Transaction ID: {}\nAmount: {}\nStatus: {}\n",
        transaction_id, amount, status
    )
}
