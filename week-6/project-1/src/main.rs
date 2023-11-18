use std::io;
fn main() {
	println!("*****Please note that this system will only run for the firsy 150 eligible candidates*****");
	let mut class_rep = String::new();
	let mut cgpa = String::new();
	let mut name = String::new();
	let mut email = String::new();
	let mut departmt = String::new();
	let mut state_of_origin = String::new();
	let mut candidate_voted = String::new();

	loop {
		println!("First of all, Are you a Class Rep? (y or n)");
		io::stdin().read_line(&mut class_rep);
		let mut classrep = class_rep.trim().to_lowercase();


		if classrep == "y" {
			println!("Alright!\n\n");
			break;
		}
			else if classrep == "n" {
			println!("Sorry but this Register is for only Class Rep\n\n\n");
			main();

		}
			else {
			println!("Please Input A Valid Response");
			main()
			} 
	}

	loop {
	println!("Secondly, Whats your CGPA?");
	io::stdin().read_line(&mut cgpa).expect("Not a valid input");
	let mut cgpa_2: f32 = cgpa.trim().parse().expect("Not a valid number");


		if cgpa_2 > 4.0 {
			println!("\n{}\nAlright!\n\n", cgpa_2);
			break;
			}	else {
			println!("Sorry but this Register is for only CGPA of above 4.0\n\n\n");
			main();
			} 
	}


	println!("Now whats your name: ");
	io::stdin().read_line(&mut name);
	let mut name_2 = name.trim().to_uppercase();

	println!("Email:  ");
	io::stdin().read_line(&mut email);
	let mail = email.trim().to_lowercase();


	loop {
	println!("\nWhich Department you in: \n\nSchool of Science and technology(SST)\n\nSchool Of Media and Mass Communication(SMMC)\n\nInstitute of Humanities(IOH)\n\n");
	io::stdin().read_line(&mut departmt);
	let mut department = departmt.trim().to_uppercase();
		if department == "SST" || department == "IOH" || department== "SMMC" {
			break;
		} else {
			println!("\n\nPlease input a valid department code");
		}
	}


	println!("\n\nWhat is your state of origin: ");
	io::stdin().read_line(&mut state_of_origin);
	let state = state_of_origin.trim().to_uppercase();


	println!("\n{}, You are eligible to vote.",name_2 );
	println!("Please enter the name of the candidate you want to vote for.");
	io::stdin().read_line(&mut candidate_voted);
	let candidate_voted2  = candidate_voted.trim();

	println!("\n\n\nAlright {}, You have succesfully voted for {}", name_2, candidate_voted2 );
	return;




	






}