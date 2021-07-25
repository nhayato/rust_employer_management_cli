use std::collections::HashMap;
use std::io;

fn main() {
    let mut department: HashMap<String, Vec<String>> = HashMap::new();
    println!("How to use");
    println!("Add: Add Amir to Sales");
    println!("List (dept): List employees belonging to HR");
    println!("List (all): List all employees");
    println!("Exit: Exit");
    println!();

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut elems = line.split_whitespace();
        // 前から切り詰めているの筋が悪い気がする
        // 大文字小文字区別してないの筋が悪い気がする
        let command = elems.nth(0).expect("no command given");

        match command {
            "Add" => {
                let name = elems.nth(0).expect("no name given");
                let dept = elems.nth(1).expect("no dept given");
                add_employee(&mut department, &name, &dept);
                println!("{:?}", department);
            }
            "List" => {
                let subcommand = elems.nth(0).expect("no subcommand given");
                match subcommand {
                    "employees" => {
                        let dept = elems.nth(2).expect("no dept geven");
                        let list = list_employee_belong_dept(&mut department, dept);
                        println!("{:?}", list);
                    }
                    "all" => {
                        let all_employees = list_sorted_all_employees(&mut department);
                        println!("{:?}", all_employees);
                    }
                    _ => (),
                }
            }
            "Exit" => break,
            _ => (),
        }
    }
}

fn add_employee(department: &mut HashMap<String, Vec<String>>, employee_name: &str, dept: &str) {
    match department.get_mut(dept) {
        Some(list) => list.push(String::from(employee_name)),
        None => {
            let v = vec![String::from(employee_name)];
            department.insert(String::from(dept), v);
        }
    }
}

fn list_employee_belong_dept(
    department: &mut HashMap<String, Vec<String>>,
    dept: &str,
) -> Vec<String> {
    match department.get(dept) {
        None => return vec![],
        Some(list) => {
            let v = list.clone();
            return v;
        }
    }
}

fn list_sorted_all_employees(department: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    let mut all_employees: Vec<String> = vec![];
    for (_, value) in department.iter() {
        for name in value {
            all_employees.push(String::from(name));
        }
    }
    all_employees
}
