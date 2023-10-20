fn main() {
	// First declare the values
	let loan:f64 = 520_000_000;
	let time = 5;
	let n = time * 12;
	let rate = 10;
	/* Formula for a = p[1+(r/100)]*n
	Formula ci = a - p
	*/

	let a = loan * (1 + ( rate / 100 )) * n;
	let ci = a - loan;
	println!("To find the compound interest, First find A");
	println!("And then subtract it by P (The principal amount)");
	println!("From the formula A = p[1+(r/100)]*n (Where n = years X 12)");
	println!("We can see that A = N{}, Therefore the Compound Interest = N{}.", a, ci); 
 
}
