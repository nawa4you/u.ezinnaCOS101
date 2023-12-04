use std::io;
fn trap () {
	let mut input1 = String::new();
	println!("\n\nOkay, please input the height of the trapezium: ");
	io::stdin().read_line(&mut input1).expect("Not a valid user string");
	let height_trap: f32 = input1.trim().parse().expect("Not a valid number");


	let mut input2 = String::new();
	println!("\n\nOkay, please input the size of the base of the trapezium: ");
	io::stdin().read_line(&mut input2).expect("Not a valid user input");
	let base1_trap: f32 = input2.trim().parse().expect("Not a valid number");
	


	let mut input3 = String::new();
	println!("\n\nPlease input the length of the top of the trapezium: ");
	io::stdin().read_line(&mut input3).expect("Not a valid user input");
	let base2_trap: f32 = input3.trim().parse().expect("Not a valid number");


	let mut area_trap: f32 = (height_trap / 2.0) * (base1_trap + base2_trap);
	println!("\n\nThe area of the trapezium: {}", area_trap);

	}
fn rhomb() {
use std::io;
	let mut input4 = String::new();
	println!("\n\nOkay, please input the first diagonal length of the rhombus: ");
	io::stdin().read_line(&mut input4).expect("Not a valid user input");
	let diag1_rhomb: f32 = input4.trim().parse().expect("Not a valid number");


	let mut input5 = String::new();
	println!("\n\nOkay, please input the second diagonal length of the rhombus: ");
	io::stdin().read_line(&mut input5).expect("Not a valid user input");
	let diag2_rhomb: f32 = input5.trim().parse().expect("Not a valid number");

	let mut area_rhomb: f32 = 0.5 * diag1_rhomb * diag2_rhomb;
	println!("\n\nThe area of the trapezium: {}", area_rhomb);
	
	
	
}

fn para() {
    use std::io;
	let mut input6 = String::new();
	println!("\n\nOkay, please input the altitude length(cm) of the parallelogram: ");
	io::stdin().read_line(&mut input6).expect("Not a valid user input");
	let alt_para: f32 = input6.trim().parse().expect("Not a valid number");


	let mut input7 = String::new();
	println!("\n\nOkay, please input the Base length(cm) of the parallelogram: ");
	io::stdin().read_line(&mut input7).expect("Not a valid user input");
	let base_para: f32 = input7.trim().parse().expect("Not a valid number");

	let mut area_para: f32 = base_para * alt_para;
	println!("\n\nThe area of the Parallelogram: {}", area_para);
	}


fn cube() {
use std::io;
	let mut input8 = String::new();
	println!("\n\nOkay, please input the length(cm) of the sides of the cube: ");
	io::stdin().read_line(&mut input8).expect("Not a valid user input");
	let l_cube: f32 = input8.trim().parse().expect("Not a valid number");

	let mut area_cube: f32 = 6.0 * (l_cube * l_cube);
	println!("\n\nThe area of the Cube: {}", area_cube);
	}

fn cyl() {
use std::io;
	let mut input9 = String::new();
	println!("\n\nOkay, please input the radius(cm) of the cylinder: ");
	io::stdin().read_line(&mut input9).expect("Not a valid user input");
	let rad_cyl: f32 = input9.trim().parse().expect("Not a valid number");


	let mut input10 = String::new();
	println!("\n\nOkay, please input the height(cm) of the cylinder: ");
	io::stdin().read_line(&mut input10).expect("Not a valid user input");
	let height_cyl: f32 = input10.trim().parse().expect("Not a valid number");

	let mut area_cyl: f32 = 3.142 * (rad_cyl * rad_cyl) * height_cyl;
	println!("\n\nThe area of the Cylinder: {}", area_cyl);
	
	
	
	
}


fn main() {

	use std::io;
	let mut input11 = String::new();
	println!("Hello! Welcome to the MTH 101 Calculator App\n\nPlease what would you like to calculate today(Input your answer as code in bracket)\n		Area of trapezium(TRA)\n		Area of rhombus(RHO)\n		Area of parallelogram(PAR)\n		Area of cube(CUB)\n		Area of cylinder(CYL)\n");
	io::stdin().read_line(&mut input11).expect("Not a valid user input");
	let calc = input11.trim().to_uppercase();

	if calc == "TRA" {
	trap();
	
	}	else if calc == "RHO" {
	rhomb();
	}	else if calc == "PAR" {
	para();
	}	else if calc == "CUB" {
	cube();
	}	else if calc == "CYL" {
	cyl();
	}	else {
		println!("\n\nIm sorry but thats a wrong input, please input a valid code(the codes in bracket)\n\n\n");
		//main();
	
	}
	
	loop{
	let mut input12 = String::new();
	println!("\n\nThank you for using the app, do you want to calculate again? (y or n) ");
	io::stdin().read_line(&mut input12).expect("Not a valid user input");
	let yorn = input12.trim().to_lowercase();
	

	if yorn == "y" {
	main();
	break;
	
	}	else if yorn == "n" {
	println!("Thanks for using!!");
	break;
	}	else {
	    println!("Please input a valid response"); 
	    }
	    

	}	
    return;

	

	




	
	
	
}