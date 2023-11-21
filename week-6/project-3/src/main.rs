use std::io;
fn main() {

	let mut input1 = String::new();
	let mut input2 = String::new();
	println!("*****Multiplication Table*****");

	println!("What number do you want to use for this Multiplication Table?");
	io::stdin().read_line(&mut input1).expect("Not a valid input");
	let number: i32 = input1.trim().parse().expect("Not a valid number");



	println!("Up to what number of multiple do you want the Table to end");
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let n: i32 = input2.trim().parse().expect("Not a valid number");

	

	for x in 1..n+1 {
	let multiple = number * x;

		println!("\n\n  ㉿{} X {} = {}",number,x, multiple);
		
		if x > n+1 {
			continue;
		}
	}
		println!("\n\n\nThat is the Multiplication table from 1 to {} of {}",n, number );


	 

}