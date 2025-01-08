// src/subscriptions/user_subscriptions.rs

use crate::subscriptions::plans::SubscriptionPlan;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSubscription {
    pub user_id: String,
    pub plan: SubscriptionPlan,
    pub start_date: u64,  // Unix timestamp
    pub end_date: u64,    // Unix timestamp
}

pub struct UserSubscriptions {
    subscriptions: HashMap<String, UserSubscription>,
}

impl UserSubscriptions {
    pub fn new() -> Self {
        UserSubscriptions {
            subscriptions: HashMap::new(),
        }
    }

    pub fn add_subscription(&mut self, user_id: &str, plan: SubscriptionPlan, start_date: u64, end_date: u64) {
        let subscription = UserSubscription {
            user_id: user_id.to_string(),
            plan,
            start_date,
            end_date,
        };
        self.subscriptions.insert(user_id.to_string(), subscription);
    }

    pub fn get_subscription(&self, user_id: &str) -> Option<&UserSubscription> {
        self.subscriptions.get(user_id)
    }

    pub fn remove_subscription(&mut self, user_id: &str) {
        self.subscriptions.remove(user_id);
    }
}
