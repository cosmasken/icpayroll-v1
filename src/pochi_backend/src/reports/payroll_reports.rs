// src/reports/payroll_reports.rs

use crate::payroll::employee::Employee;

pub fn generate_payroll_report(employees: Vec<Employee>, month: &str) -> String {
    let mut report = format!("Payroll Report for {}\n", month);
    
    for employee in employees {
        report.push_str(&format!(
            "Employee: {}\nRole: {}\nSalary: {}\n\n",
            employee.name, employee.role, employee.salary
        ));
    }
    
    report
}
