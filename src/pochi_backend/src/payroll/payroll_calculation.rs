// src/payroll/payroll_calculation.rs

use crate::employee_management::employee::Employee;

pub fn calculate_salary(employee: &Employee, hours_worked: u32, hourly_rate: u128) -> u128 {
    let base_salary = hours_worked as u128 * hourly_rate;
    let salary_after_deductions = apply_deductions(employee, base_salary);
    let salary_after_bonuses = apply_bonuses(employee, salary_after_deductions);

    salary_after_bonuses
}

fn apply_deductions(employee: &Employee, salary: u128) -> u128 {
    // Example: Deducting 10% for tax
    let tax_deduction = salary * 10 / 100;
    salary - tax_deduction
}

fn apply_bonuses(employee: &Employee, salary: u128) -> u128 {
    // Example: Adding a bonus of 5% of the salary for certain employees
    if employee.role == "Manager" {
        salary + salary * 5 / 100
    } else {
        salary
    }
}
