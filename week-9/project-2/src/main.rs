use std::io::Write;

fn main() {
	let sn = [3,4,5,6,7];
	let sname = ["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
	let matric = ["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
	let dept = ["Accounting","Economics","Computer","Electrical","Mechanical"];
	let lvl = [300,100,200,200,100];
	println!("Creating File...\n\n\n");

	let mut file = std::fs::File::create("PAU-SIMS.txt").expect("Cant create file");
	// file.write_all("".as_bytes());


	writeln!(file, "|{:>10}| {:>20}", "1","****Pan-Atlantic University Student Information Management System****");

	writeln!(file, "|{:>10}| {:>20}| {:>20}| {:>20}| {:>15}|", "2","Student Name", "Matric. Number","Department","Level");

	for i in 0..sn.len() {

	writeln!(file, "|{:>10}| {:>20}| {:>20}| {:>20}| {:>15}|", sn[i],sname[i],matric[i],dept[i],lvl[i]).expect("Couldnt create file");

	}	println!("File created Sussesfully (@ PAU-SIMS)");
}