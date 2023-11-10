use std::io;
fn main () {
	println!("A ROOT FINDING PROGRAM");
	println!("Please input a: " );
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid input");
	let a: i32 = input1.trim().parse().expect("Not a valid number");


	println!("Please input b: ");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let b: i32 = input1.trim().parse().expect("Not a valid number");

	println!("Please input c: ");
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("Not a valid input");
	let c: i32 = input1.trim().parse().expect("Not a valid number");

	//now we "if" it

	if b.pow(2) > 4 * (a * c) {
		println!("There are two distinct root in the eqn");
	}
	else if b.pow(2) == 4 * (a * c) {
		println!("There is exactly one root in the eqn");
		}
	else if b.pow(2) < 4 * (a * c) {
		println!("There are no real root in the eqn");
		}
	else {
		println!("I dont know");
	}
	
	
	
	
	
	
}


