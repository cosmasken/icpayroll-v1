// src/onramp_offramp/crypto.rs

use crate::onramp_offramp::providers::PaymentProvider;
use crate::onramp_offramp::thunes::ThunesClient;

pub async fn crypto_to_fiat(provider: &PaymentProvider, crypto_amount: u128, currency: &str) -> Result<Vec<u8>, String> {
    match provider {
        PaymentProvider::Thunes(client) => {
            client.offramp_crypto_to_fiat(crypto_amount, currency).await
        }
        // Add other providers logic here
    }
}
