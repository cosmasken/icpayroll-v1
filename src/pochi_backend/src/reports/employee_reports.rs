// src/reports/employee_reports.rs

use crate::payroll::employee::Employee;

pub fn generate_salary_report(employee: &Employee, hours_worked: u32) -> String {
    let total_salary = if hours_worked > 160 {
        employee.salary + (employee.hourly_rate * (hours_worked - 160) as u128)
    } else {
        employee.salary
    };
    
    format!(
        "Employee: {}\nRole: {}\nSalary: {}\nHours Worked: {}\nTotal Salary: {}\n",
        employee.name, employee.role, employee.salary, hours_worked, total_salary
    )
}
