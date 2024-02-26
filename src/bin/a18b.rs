// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//

// * Use an enum to represent all types of employees
enum EmployeeType {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    employee_type: EmployeeType,
    employed: bool,
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn can_access_building(employee: &Employee) -> Result<bool, String> {
    match employee {
        Employee {
            employed: false, ..
        } => Err("Employee is not employed".to_owned()),
        Employee {
            employee_type: EmployeeType::Maintenance,
            ..
        } => Ok(true),
        Employee {
            employee_type: EmployeeType::Marketing,
            ..
        } => Ok(true),
        Employee {
            employee_type: EmployeeType::Manager,
            ..
        } => Ok(true),
        _ => Ok(false),
    }
}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access_result(employee: &Employee) -> Result<(), String> {
    let result = can_access_building(employee)?;
    println!("Employee may access building: {}", result);
    Ok(())
}

fn main() {
    let employee1 = Employee {
        employee_type: EmployeeType::Maintenance,
        employed: false,
    };
    let employee2 = Employee {
        employee_type: EmployeeType::Marketing,
        employed: true,
    };
    let employee3 = Employee {
        employee_type: EmployeeType::LineSupervisor,
        employed: true,
    };
    let employee4 = Employee {
        employee_type: EmployeeType::Manager,
        employed: true,
    };
    let employee5 = Employee {
        employee_type: EmployeeType::KitchenStaff,
        employed: true,
    };
    let employee6 = Employee {
        employee_type: EmployeeType::AssemblyTechnician,
        employed: true,
    };
    let employee7 = Employee {
        employee_type: EmployeeType::Maintenance,
        employed: true,
    };

    let employees = vec![
        employee1, employee2, employee3, employee4, employee5, employee6, employee7,
    ];

    for employee in employees {
        match print_access_result(&employee) {
            Err(e) => println!("Error: {}", e),
            _ => (),
        };
    }
}
