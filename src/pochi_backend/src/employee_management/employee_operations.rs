// src/employee_management/employee_operations.rs

use crate::employee_management::employee::Employee;
use std::collections::HashMap;

pub struct EmployeeDatabase {
    employees: HashMap<String, Employee>,
}

impl EmployeeDatabase {
    pub fn new() -> Self {
        EmployeeDatabase {
            employees: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.insert(employee.id.clone(), employee);
    }

    pub fn get_employee(&self, employee_id: &str) -> Option<&Employee> {
        self.employees.get(employee_id)
    }

    pub fn update_employee(&mut self, employee_id: &str, new_employee: Employee) {
        if let Some(employee) = self.employees.get_mut(employee_id) {
            *employee = new_employee;
        }
    }

    pub fn delete_employee(&mut self, employee_id: &str) {
        self.employees.remove(employee_id);
    }

    pub fn get_all_employees(&self) -> Vec<Employee> {
        self.employees.values().cloned().collect()
    }
}
