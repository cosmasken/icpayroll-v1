// src/taxation_compliance/compliance_reports.rs

use crate::taxation_compliance::tax_calculations;

pub fn generate_tax_report(income: f64, country: &str) -> String {
    let tax = tax_calculations::calculate_tax(income, country);
    format!(
        "Tax Report: \nCountry: {}\nIncome: {}\nTax Deducted: {}",
        country, income, tax
    )
}
