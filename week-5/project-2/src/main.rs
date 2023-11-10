use std::io;
fn main() {
   let mut incentive: u32;

   println!("Welcome to the company!\nPlease input your age, so we can determine your incentive\n");
   println!("\n");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Not a valid input");
   let age: u32 = input1.trim().parse().expect("Not a valid number");

	println!("Great, now we have to know if you are experienced in the job\nAre you? (y or n)");
	println!("\n");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let expert = input2.trim().to_lowercase();
   //exp and age of an employee to determine the annial incentive with a few if data
	loop {
	if expert == "n" {
	incentive = 100_000;
	break;
	} else if expert == "y" && age >= 40 {
	incentive = 1_560_000;
	break;
	} else if expert == "y" && age >=30 && age < 40 {
	incentive = 1_480_00;
	break;
	} else if expert == "y" && age < 28 {
	incentive = 1_300_000;
	break;
	} else {
	println!("Sorry, unavailable spots for you :(");
	}
	} println!("Congratulations, we have a spot at N{} for you!", incentive );
}
