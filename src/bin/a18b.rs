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
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Employees {
    MaintenanceCrews,
    MarketingDepartmentEmployees,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}

enum Status {
    Active,
    Terminated
}

struct Employee {
    employee: Employees,
    employed: Status
}

fn try_access(e: &Employee) -> Result<(), String> {
    match e.employed {
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => ()
    }

    match e.employee {
        Employees::MaintenanceCrews => Ok(()),
        Employees::MarketingDepartmentEmployees => Ok(()),
        Employees::Managers => Ok(()),
        _ => Err("Invalid Position".to_owned())
    }
}

fn print_access(e: &Employee) -> Result<(), String> {
    let _attempt_access = try_access(e)?;
    println!("Access ok");
    Ok(())
}

fn main() {
    let e: Vec<Employee> = vec![
        Employee {
            employee: Employees::MaintenanceCrews,
            employed: Status::Active
        },
        Employee {
            employee: Employees::MarketingDepartmentEmployees,
            employed: Status::Terminated
        },
        Employee {
            employee: Employees::Managers,
            employed: Status::Active
        },
        Employee {
            employee: Employees::LineSupervisors,
            employed: Status::Terminated
        },
        Employee {
            employee: Employees::KitchenStaff,
            employed: Status::Terminated
        },
        Employee {
            employee: Employees::AssemblyTechnicians,
            employed: Status::Active
        }
    ];

    for i in e {
        match print_access(&i) {
            Err(e) => println!("Access denied: {}", e),
            _ => ()
        }
    }
}
