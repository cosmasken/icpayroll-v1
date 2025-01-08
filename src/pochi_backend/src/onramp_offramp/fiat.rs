// src/onramp_offramp/fiat.rs

use crate::onramp_offramp::providers::PaymentProvider;
use crate::onramp_offramp::thunes::ThunesClient;

pub async fn fiat_to_crypto(provider: &PaymentProvider, fiat_amount: u128, currency: &str) -> Result<Vec<u8>, String> {
    match provider {
        PaymentProvider::Thunes(client) => {
            client.onramp_fiat_to_crypto(fiat_amount, currency).await
        }
        // Add other providers logic here
    }
}
