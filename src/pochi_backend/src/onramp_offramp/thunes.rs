// src/onramp_offramp/thunes.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::http_outcall::{http_outcall, HttpRequestData};

#[derive(Serialize, Deserialize)]
pub struct ThunesClient {
    api_key: String,
    base_url: String,
}

impl ThunesClient {
    pub fn new(api_key: &str) -> Self {
        ThunesClient {
            api_key: api_key.to_string(),
            base_url: "https://api.thunes.com".to_string(),
        }
    }

    pub async fn onramp_fiat_to_crypto(&self, fiat_amount: u128, currency: &str) -> Result<Vec<u8>, String> {
        let url = format!("{}/onramp/convert", self.base_url);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let body = serde_json::json!({
            "fiat_amount": fiat_amount,
            "currency": currency
        }).to_string();

        // Use the HTTP outcall function from utils module
        let response = http_outcall(HttpRequestData {
            url,
            body,
            headers,
        }).await;

        response.map_err(|e| format!("Error making HTTP outcall: {:?}", e))
    }

    pub async fn offramp_crypto_to_fiat(&self, crypto_amount: u128, currency: &str) -> Result<Vec<u8>, String> {
        let url = format!("{}/offramp/convert", self.base_url);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let body = serde_json::json!({
            "crypto_amount": crypto_amount,
            "currency": currency
        }).to_string();

        // Use the HTTP outcall function from utils module
        let response = http_outcall(HttpRequestData {
            url,
            body,
            headers,
        }).await;

        response.map_err(|e| format!("Error making HTTP outcall: {:?}", e))
    }
}
