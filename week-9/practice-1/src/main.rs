use std::io::Write;
fn main() {
	let announce = "Week 9\n";
	let dept = "Department of Computer Science";

	let mut file = std::fs::File::create("data.txt").expect("Couldnt create file");

	file.write_all("Welcome to rust programming\n".as_bytes()).expect("Couldnt write file");
	file.write_all(announce.as_bytes()).expect("Couldnt write file");
	file.write_all(dept.as_bytes()).expect("Couldnt write file");
}
