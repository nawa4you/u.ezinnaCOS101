// #[allow(unused_must_use)]
// #[allow(non_snake_case)]


// use std::io;
// fn main() {
// 	// //first find distance from the given set of data
// 	// //then find the other one 
// 	// //then ask if the user wants to imput another data (loop)
// 	// //end


// 	// let mut Speed = String::new();
// 	// let mut Time = String::new();
// 	// let mut Unit = String::new();

// 	// println!("HELLO! THIS IS TO A DISTANCE CALCULATOR APP.");
// 	// println!(" ");
// 	// println!(" ");
// 	// println!("PLEASE INPUT A SPEED ");
// 	// io::stdin().read_line(&mut Speed).expect("Not a valid input");
// 	// let speed: f32 = Speed.trim().parse().expect("Not a valid number");

//  //    println!("PLEASE INPUT TIME");
//  //    io::stdin().read_line(&mut Time).expect("Not a valid input");
//  //    let time: f32 = Time.parse().expect("Not a valid number");


// 	// //check if speed in km or m
// 	// println!("IS YOUR SPEED IN KILOMETERS OR IN MILES? (KM or M" );
// 	// io::stdin().read_line(&mut Unit).expect("Not a valid input");
// 	// let unit: &str = &Unit.to_lowercase();

// 	// let mut speedKm = speed;
// 	// if unit == "m" {
// 	// 	speedKm *= 1.609;
// 	// }
// 	// println!("From the data given, Distance travelled by car = ");
// 	// println!("{}",speedKm );
// 	// println!(" ");
// 	// let distance: f32 = speedKm * time;
// 	// println!("And the distance travelled => {} * {} = {}", speedKm, time, distance);

// 	// loop {
// 	// 	println!("Would you like to calculate something again? (y or n)");
// 	// 	let mut Restart = String::new();
// 	// 	io::stdin().read_line(&mut Restart);
// 	// 	let restart: &str = &Restart.to_lowercase();

// 	// 	if restart == "y" {
// 	// 	main();
// 	// 	break;
// 	// 		}
// 	// 	else if restart == "n" {
// 	// 		println!("THANK YOU FOR USING(made by daniel(for hours))");
// 	// 		break;
// 	// 		}
// 	// 	else {
// 	// 		println!("Please enter a valid input (y or n)");
// 	// 		}
		
// 	// }

// 	let mut speed: f32 = 80.0;
// 	let mut speedKm: f32 = 1.609 * speed;
// 	let mut time: f32 = 2.0;
// 	let mut timeSec: f32 = time * 60.0 * 60.0 ; 
// 	let mut distance: f32 = speedKm * timeSec;
// 	println!("For the first calculation, The distance when speed is {}Miles and time is {}Hours",speed, time);
// 	println!("Distance => Speed(Km) X Time(Sec)");
// 	println!("Distance = {}", distance);
// 	speed: f32 = 120.0;
// 	time: f32 = 4.0;
// 	println!("For the next calculation, speed is {}miles and time is {}hours", speed, time);
// 	println!("Distance => Speed(Km) X Time(Sec)");
// 	println!("Distance = {}", distance);





// }

use std::io;

fn main() {
    let mut speed: f32;
    let mut input: String;
    let mut t1: f32;
    let mut restart = String::new();

    println!("CALCULATOR APP FOR SPEED OF CARS (IN KILOMETERS)");

    // Get speed input and convert to f32
    io::stdin().read_line(&mut speed).expect("Not a valid Input");
    speed = speed.trim().parse().expect("Not a valid Number");

    // Check speed unit and convert to kilometers if necessary
    loop {
        println!("\nIs your speed in KILOMETERS or MILES\n(K or M?)");
        io::stdin().read_line(&mut input).expect("Not a valid Input");
        input = input.trim().to_lowercase();

        if input == "k" {
            break;
        } else if input == "m" {
            speed *= 1.60934;
            break;
        } else {
            println!("INPUT EITHER K OR M PLEASE");
        }
    }

    // Get time input and convert to f32
    io::stdin().read_line(&mut t1).expect("Not a valid Input");
    t1 = t1.trim().parse().expect("Not a valid Number");

    // Check time unit and convert to seconds
    loop {
        println!("\nNow input the time");
        io::stdin().read_line(&mut t1).expect("Not a valid Input");
        t1 = t1.trim().parse().expect("Not a valid Number");

        println!("Is the time in seconds, hours or minutes (s, h, m?)");
        input = String::new();
        io::stdin().read_line(&mut input).expect("Not a valid Input");
        input = input.trim().to_lowercase();

        if input == "s" {
            break;
        } else if input == "h" {
            t1 *= 3600.0;
            break;
        } else if input == "m" {
            t1 *= 60.0;
            break;
        } else {
            println!("Please input a valid Number");
        }
    }

    // Calculate total distance
    let distance: f32 = speed * t1;

    // Display result
    println!("Given your inputs, speed as {}km and time as {}, the total distance of the car = {}", speed, t1, distance);

    // Restart prompt
    loop {
        println!("\nDO YOU WANT TO CALCULATE AGAIN? y or n");
        io::stdin().read_line(&mut restart).expect("Not a valid inout");
        restart = restart.trim().to_lowercase();

        if restart == "y" {
            main();
            break;
        } else if restart == "n" {
            println!("THANKS FOR USING (made by daniel)");
            break;
        } else {
            println!("Not a valid input");
        }
    }
}
