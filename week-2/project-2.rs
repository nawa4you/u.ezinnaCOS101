fn main() {
	// Def your values
	let t:f64 = 450_000.00 * 2.0;
	let m:f64 = 1_500_000.00 * 1.0;
	let h:f64 = 750_000.00 * 3.0;
	let d:f64 = 2_850_000.00 * 3.0;
	let a:f64 = 250_000.00 * 1.0;
	let sum:f64 = t + m + h + d + a;
	let avg = sum / 10.0;

	// Now find a way to add the quantity to them and multiply
	println!("First we multiply each Qty by the amount to find sum");
	println!("From the given sales record, total sum = {}", sum );
	println!("And given that number of product = 10, Average = sum / 10 = {}", avg);


	
}
