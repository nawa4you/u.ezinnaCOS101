use std::io::OpenOptions;
use std::io::Write;

fn main() {
	let mut file = OpenOptions::new().append(true).open("data.txt").expect("cant create file");
	file.write_all("\nHello Class".as_bytes());
	file.write_all("\nThis is the appendage of the document".as_bytes());


	println!("File appended succesfully");

}