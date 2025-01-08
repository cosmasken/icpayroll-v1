use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

const MAX_VALUE_SIZE: u32 = 500;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub role: String,
    pub salary: u128, // Salary in cents to avoid floating-point precision issues
    pub payment_method: String, // e.g., "bank_transfer", "crypto"
    pub status: String, // e.g., "active", "inactive"
}

impl Storable for Employee {
    pub fn new(id: &str, name: &str, role: &str, salary: u128, payment_method: &str, status: &str) -> Self {
        Employee {
            id: id.to_string(),
            name: name.to_string(),
            role: role.to_string(),
            salary,
            payment_method: payment_method.to_string(),
            status: status.to_string(),
        }
    }

    pub fn update_salary(&mut self, new_salary: u128) {
        self.salary = new_salary;
    }

    pub fn update_status(&mut self, new_status: &str) {
        self.status = new_status.to_string();
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}