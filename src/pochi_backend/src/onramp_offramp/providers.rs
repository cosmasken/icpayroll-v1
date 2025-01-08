// src/onramp_offramp/providers.rs

use crate::onramp_offramp::thunes::ThunesClient;

pub enum PaymentProvider {
    Thunes(ThunesClient),
    // Add other providers as needed
}

impl PaymentProvider {
    pub fn new_thunes(api_key: &str) -> Self {
        PaymentProvider::Thunes(ThunesClient::new(api_key))
    }

    // Other provider initialization logic
}
