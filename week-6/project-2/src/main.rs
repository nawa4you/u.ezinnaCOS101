use std::io;
fn main() {
	println!("*****Researchers Publicationlet incentive System*****");
	println!("*****THIS PROGRAM WILL ONLY RUN FOR THE FIRST 500 RESEARCHERS*****");

	let mut input1 = String::new();
	let mut input2 = String::new();

		println!("Please What is your name?");
		io::stdin().read_line(&mut input1);
		let name = input1.trim().to_uppercase();
 
	println!("{}, how many papers do you want to publish?", name);
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let papers: i32 = input2.trim().parse().expect("Not a valid number");

	//let mutlet incentive: i32;

	if papers >=3 && papers <=5 {
	let incentive = 500_000;
		println!("\n\nAlright {}\n\nBased on your number of pages published, your incentive will be a total of N{}",name,incentive);

		
		}
		else if papers >5 && papers <10 {
		let incentive = 800_000;
			println!("\n\nAlright {}\n\nBased on your number of pages published, your incentive will be a total of N{}",name,incentive);

			


		}
			else if papers >=10 {
		let incentive = 1_000_000;
			println!("\n\nAlright {}\n\nBased on your number of pages published, your incentive will be a total of N{}", name,incentive);

					


		}
		else if papers <3 && papers >0 {
		let incentive = 100_000;
			println!("\n\nAlright {}\n\nBased on your number of pages published, your incentive will be a total of N{}", name,incentive);




		}
		else {
			println!("Sorry please try again");
			main();
		}

}