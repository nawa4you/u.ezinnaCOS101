fn main() {
    let fullname = "Daniel Ezinna";
    let department = "Computer Science";
    let uni = "PAU";

    let mut school = "School of science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is: {}", fullname );
    //check length
    println!("The length of my fullname is: {}", fullname.len()-1);
    println!("I am a student of {} department", department );
    println!("{}",school );
    println!("{}",uni );
}
