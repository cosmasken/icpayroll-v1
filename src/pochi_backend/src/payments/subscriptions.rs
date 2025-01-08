use std::collections::HashMap;

#[derive(Default)]
pub struct SubscriptionManager {
    pub subscriptions: HashMap<u64, Vec<Subscription>>, // User ID -> List of Subscriptions
}

#[derive(Clone, Debug)]
pub struct Subscription {
    pub name: String,
    pub price: u128,
    pub duration: u64, // Duration in seconds
    pub start_time: u64,
    pub end_time: u64,
}

impl SubscriptionManager {
    pub fn add_subscription(
        &mut self,
        user_id: u64,
        name: String,
        price: u128,
        duration: u64,
    ) -> Result<(), String> {
        let start_time = ic_cdk::api::time();
        let end_time = start_time + duration;

        let subscription = Subscription {
            name,
            price,
            duration,
            start_time,
            end_time,
        };

        self.subscriptions
            .entry(user_id)
            .or_default()
            .push(subscription);

        Ok(())
    }

    pub fn get_active_subscriptions(&self, user_id: u64) -> Vec<Subscription> {
        self.subscriptions
            .get(&user_id)
            .unwrap_or(&Vec::new())
            .clone()
            .into_iter()
            .filter(|sub| sub.end_time > ic_cdk::api::time())
            .collect()
    }
}
