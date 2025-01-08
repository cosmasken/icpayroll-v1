pub mod processor;
pub mod balances;
pub mod subscriptions;

use ic_cdk_macros::update;

// High-level API for payments
#[update]
fn make_payment(user_id: u64, amount: u128, description: String) -> Result<(), String> {
    payments::processor::process_payment(user_id, amount, description)
}
