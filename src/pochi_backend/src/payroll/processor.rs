// src/payroll/processor.rs

use crate::subscriptions::payment::process_subscription_payment;
use crate::payroll::employee::Employee;
use crate::onramp_offramp::providers::PaymentProvider;
use crate::onramp_offramp::thunes::ThunesClient;

pub async fn process_payroll_payment(
    provider: &PaymentProvider,
    employee: &Employee,
    amount: u128,
) -> Result<String, String> {
    // Logic for processing salary payment (integrating with Thunes or another payment provider)
    let result = process_subscription_payment(provider, &employee.id, &amount).await;

    match result {
        Ok(_) => Ok(format!("Payroll payment successful for employee: {}", employee.name)),
        Err(err) => Err(format!("Payroll payment failed: {}", err)),
    }
}
