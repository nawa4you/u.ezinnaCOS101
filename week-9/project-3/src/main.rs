use std::io::Write;
fn main() {
	let sn = [1,2,3,4,5];

	let commisioners = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu","Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye" ];

	let ministry = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];

	let zone = ["South West","North East","South South","South West","South East"];
	let mut file = std::fs::File::create("Convicted-Ministers.txt").expect("Couldnt Create file");


	file.write_all("This is a backup file to the lost documents of convicted ministers and their states: \n\n".as_bytes()).expect("couldnt write");


	writeln!(file,"{:>10}| {:>30} {:>20} {:>20}", "S/N","Name of commisioner","Ministry","Geopolitical Zones");


	for i in 0..sn.len() {

	writeln!(file,"{:>10}| {:>30} {:>20} {:>20}", sn[i],commisioners[i],ministry[i],zone[i]);

	}	println!("File Sussesfully created(@ Convicted-Ministers)");




}