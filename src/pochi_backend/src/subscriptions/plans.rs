// src/subscriptions/plans.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPlan {
    pub id: String,
    pub name: String,
    pub price: u128, // Price in smallest unit (e.g., cents, satoshis)
    pub duration_days: u32, // Duration in days
}

impl SubscriptionPlan {
    pub fn new(id: &str, name: &str, price: u128, duration_days: u32) -> Self {
        SubscriptionPlan {
            id: id.to_string(),
            name: name.to_string(),
            price,
            duration_days,
        }
    }
}

// Example of predefined plans
pub fn get_default_plans() -> Vec<SubscriptionPlan> {
    vec![
        SubscriptionPlan::new("basic", "Basic Plan", 1000, 30),  // 1000 represents the price in smallest unit
        SubscriptionPlan::new("premium", "Premium Plan", 5000, 30),
    ]
}
