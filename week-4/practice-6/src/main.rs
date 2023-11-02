fn main() {
    let n1 = "Electrical ".to_string();
    let n2 = "Electronic ".to_string();
    let n3 = "Engineering ".to_string();
    let n4 = n1 + &n2 + &n3; //refference is passed 

    // About EE
    println!("\n The {} is the information by the aspiratuion to train electrical Engineering proffesionals in the areas of design, building and maintenance of electrical control systems. ",n4 );
    let w1 = "Computer ".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2; //w2 refference is passed\
    println!();
    println!("{} is aimed at developing competent, creative, innovative and ethically-minded persons, capable of creating value in the diverse fields of Computer Science. ",w3 );
    
}
