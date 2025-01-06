struct Employee {
    name: String,
    id: String,
}

impl Employee {
    fn create_employee(name: String, id: String) -> Self {
        return Self { name, id };
    }

    fn print_emp_data(self: &Self) {
        println!("employee name is = {}\nand id is {}", self.name, self.id);
    }
}

fn main() {
    let emp1: Employee = Employee::create_employee(String::from("skd"), String::from("20"));
    emp1.print_emp_data();
}
