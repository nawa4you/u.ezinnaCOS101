use std::io::{self, Read};

fn main() {
    println!("***Globacom Database Access Point***\n");

    let mut input1 = String::new();
    println!("Please input your position:\n1) Admin\n2) Project Manager\n3) Employee\n4) Customer\n5) Vendor");

    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let position: u32 = input1.trim().parse().expect("Not a valid number");

    match position {
        1 => admin(),
        2 => pmgr(),
        3 => emp(),
        4 => cust(),
        5 => vend(),
        _ => println!("Please input a valid number"),
    }
}

fn admin() {
    let mut dbase_structure = std::fs::File::open("databaseStructure.sql").unwrap();
    let mut contents = String::new();
    dbase_structure.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn pmgr() {
    let mut project_tb = std::fs::File::open("project.sql").unwrap();
    let mut contents = String::new();
    project_tb.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn emp() {
    let mut staff_tb = std::fs::File::open("staff.sql").unwrap();
    let mut contents = String::new();
    staff_tb.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn cust() {
    let mut customer_tb = std::fs::File::open("customer.sql").unwrap();
    let mut contents = String::new();
    customer_tb.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn vend() {
    let mut data_plan = std::fs::File::open("dataplan.sql").unwrap();
    let mut contents = String::new();
    data_plan.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
