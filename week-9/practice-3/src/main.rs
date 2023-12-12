use std::fs;
fn main() {
	fs::remove_file("data.txt").expect("Cant delete file");
	println!("File is removed");


}
