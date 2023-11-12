//program to make an order from a menu
use std::io;
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(non_snake_case)]

fn main() {

	let mut total: f32 = 0.0;
	let mut discTotal: f32 = 0.0;
	let mut price:f32 = 0.0;
	//let mut choice = "";
	//let mut choice: &str;
	let mut portions:f32 = 0.0;
	let p = "Poundo Yam/Edinkaiko Soup";
	let w = "White Rice & Stew";
	let f = "Fried Rice & Chicken";
	let a = "Amala & Ewedu Soup";
	let e = "Eba & Egusi Soup";


		println!("WELCOME TO OUR KITCHEN, HERES A LIST OF THINGS WE SELL (input the characters by the left to pick a meal)\n
		P = Poundo Yam/Edinkaiko Soup 		- N3,200\n
		F = Fried Rice & Chicken 		- N3,000\n
		A = Amala & Ewedu Soup			- N2,500\n
		E = Eba & Egusi Soup			- N2,000\n
		W = White Rice & Stew			- N2,500\n
		");

		//define the choice and price from input
		let mut input1 = String::new();
		io::stdin().read_line(&mut input1);
		let meal = input1.trim().to_lowercase();
		println!("Good choice, How many Portions do you want?");
		let mut input2 = String::new();
		io::stdin().read_line(&mut input2).expect("Not a valid input");
		let portions:f32 = input2.trim().parse().expect("Not a valid number");
		if meal == "f" {
			let portions:f32 = portions * 3000.0;
			println!("Ahh {}, A good option!, ", f);

			
				} else if meal == "a" { 
			let portions:f32 = portions * 2500.0;
			println!("Ahh {}, A good option!", a);


			
				} else if meal == "e" { 
			let portions:f32 = portions * 2000.0;
			println!("Ahh {}, A good option!", e);


			
					} else if meal == "p" { 
			let portions:f32 = portions * 3200.0;
			println!("Ahh {}, A good option!", p);
	
					} else if meal == "w" { 
			let portions:f32= portions * 2500.0;
			println!("Ahh {}, A good option!", w);
			
					} else { 
			println!("Please input a single letter( First letter of the food options");
			main();
		}
	

	

	let total:f32 = portions;

	//the if code


	if total >10_000.0 {
		let discTotal: f32 = total - ((5.0 / 100.0) * total );
		println!("Great, for you order, {} portion of is a total of N{}\n",portions, total );
		println!("And congratulations, you have been given a 5% discount\nYou made a purchase above N10,000\n\nTotal price for your order is now:  N{}", discTotal );
		}    
    else {
		println!("Great, for you order, {} portion of is a total of N{}\n",portions, total );
	}
}
