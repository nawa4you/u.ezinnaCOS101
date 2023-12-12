use std::io;
fn main() {
	let aps1_2 = vec!["Intern", "-", "Paralegal", "Placement"]; 
	let aps3_5 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]; 
	let aps5_8 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"]; 
	let el1_8_10 = vec!["Office manager", "Post-Doc Researcher", "Senior Associate 1-2", "Placement"]; 
	let el2_10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]; 
	let apsSES = vec!["CEO", "Dean", "Partner", "Principal"]; 

	println!("*****Welcome to the Public Service APS level checker*****\n\nWhat is your line of field:\n\n		Office Administrator(OFA)\n\n		Academic(ACA)\n\n		Lawyer(LAW)\n\n		Teacher(TCH)\n\n");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid input");
	let mut exp: u16 = input1.trim().parse().to_lowercase().expect("Not a valid APS");


	loop {
		println!("Please how many years of experience do you have (Between 1 - 13 )\n\n");
		let mut input2 = String::new();
		io::stdin().read_line(&mut input1).expect("Not a valid input");
		let mut field = input1.trim().to_lowercase().expect("Not a valid APS");
		
	}








}


fn print() {
	yo
}