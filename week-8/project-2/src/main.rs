use std::io;
fn main() {

	let mut names: Vec<String > = Vec::new();
	let mut years: Vec< u32> = Vec::new();



	println!("*****Welcome to Ernst & Young (EY) Global Limited\n\nPlease how many people are going to apply for the program\n\n");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid input");
	let mut people_num: u16 = input1.trim().parse().expect("Not a valid amount");


	println!("Alright, {} people\n\n", people_num);


	println!("Now please enter the names of the people that want to use the program: ");


	for i in  0..people_num {

	let mut input2 = String::new();
	print!("{}: ",i+1 );
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let mut ppl_name = input2.trim().to_uppercase();
	names.push(ppl_name);

	println!("\n");
	let mut input3 = String::new();
	print!("Years of experience: ");
	io::stdin().read_line(&mut input3).expect("Not a valid input");
	let mut ppl_exp: u16 = input3.trim().parse().expect("Not a valid number of years");
	years.push(ppl_exp);
	println!("\n\n");

	
	}




	if let mut highest_year = *years.iter().max().unwrap() {
		let highest_year = years[highest_year];
		let highest_name = *names[highest_year];
	println!("Alright now from the given data set\n\nThe person with the highest number of years of experience is: {}\nWith {} years of experience.", highest_name,highest_year);
	}	else {
		println!("No names entered\n\n");
		main();
	}














	// for i in 0..count.len() {
	// 	println!("{}: {}",vec2[i], count[i] );

	// 	if i > count+1{
	// 			continue;
	// 	}

	// }



}
