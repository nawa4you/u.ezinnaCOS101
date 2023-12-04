use std::io;

fn main() {

    println!("ERNEST & YOUNG GLOBAL LIMITED NIGERIA");
    let mut max_name = String::new();
    let mut max_experience = 0;
    
    let mut input1 = String::new();
    println!("How many developers are applying:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let dev_num = input1.trim().parse().expect("invalid input");


    for i in 0..dev_num{
        let mut experience = 0;

        let mut name = String::new();
        println!("Enter developer name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Enter years of experience:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        experience = input.trim().parse().expect("Invalid input");


    println!("What state are you from?");
    let mut state = String::new();
    io::stdin().read_line(&mut state).expect("Failed to read line");

        println!("Why do you want to be selected for this job?");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("Failed to read line");
        println!("\nThank you for your input. Mr/Mrs {} from {}", name,state);



        // Check if this developer has more experience than the current max
        if experience > max_experience {
            max_name = name.trim().to_string();
            max_experience = experience;

        }
    }

    // Display the developer with the highest years of experience
    if max_experience > 0 {
        println!("The developer with the highest years of experience is: {} with {} years ", max_name, max_experience);
    } else {
        println!("No developers found");
    }
}