use std::io::Write;
fn main() {
	println!("Creating File....\n\n\n");
	let sn = [1,2,3,4,5,6];
	let lager = ["33 Export", "Desperados","Goldberg","Gulder","Heineken","Star"];
	let stout = ["Legend","Turbo King","Williams", "-", "-", "-"];
	let NonAlc = ["Maltina","Amstel Malta","Mata Gold","Fayrouz", "-", "-"];


	let mut file = std::fs::File::create("Data.txt").expect("Cant create file");

	file.write_all("Nigerian Breweries Plc\nPortfolio of high-quality Lager, Stout, Non-alcoholics and Spirit\n\n\n".as_bytes());

	writeln!(file, "{:>10},{:>20},{:>20},{:>20}", "S/N","Lagers","Stout","Non-Alcoholic");


	for i in 0..stout.len() {

		writeln!(file, "{:>10},{:>20},{:>20},{:>20}", sn[i], lager[i], stout[i], NonAlc[i]);
	}


		println!("File Created Succesfully");


}