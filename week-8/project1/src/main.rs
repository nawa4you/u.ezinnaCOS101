use std::io;
fn main(){

    println!("APS LEVEL CHECKER by the Fedral Goverment of Nigeria");

let mut input1 = String::new();
println!("How many staff do you want to enter? ");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let staff_num = input1.trim().parse().expect("invalid input");

let mut name: Vec<String> = Vec::new();
// let mut experience = Vec::new();
let mut occupation: Vec<String> = Vec::new();
let mut rank: Vec<String> = Vec::new();
let mut position: Vec<&str> = Vec::new();

for i in 1..=staff_num{

    println!("\nStaff No.{}",i);

    let mut input2 = String::new();
    println!("\nEnter staff name:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let staff_name = input2.trim().to_uppercase();
    

    let mut input3 = String::new();
    println!("\nStaff Department (Office administrator, Academic, Lawyer, Teacher): ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let staff_occu = input3.trim().to_uppercase();

    if staff_occu != "OFFICE ADMINISTRATOR" && staff_occu != "ACADEMIC" &&
    staff_occu != "LAWYER" && staff_occu != "TEACHER"{

         println!("INVALID INPUT");
        break;
    }

    if staff_occu == "OFFICE ADMINISTRATOR"{

        let mut input4 = String::new();
        println!("\nStaff Occupation (Intern/Administrator/senior administrator/Office manager/Director/CEO): ");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let staff_rank = input4.trim().to_uppercase();

        if staff_rank == "INTERN"{
           position.push("APS 1-2");
        }
        else if staff_rank == "ADMINISTRATOR"{
            position.push("APS 3-5");
        }
        else if staff_rank == "SENIOR ADMINISTRATOR"{
            position.push("APS 5-8");
        }
        else if staff_rank == "OFFICE MANAGER"{
            position.push("EL1 8-10");
        }
        else if staff_rank == "DIRECTOR"{
            position.push("EL2 10-13");
        }
        else if staff_rank == "CEO"{
            position.push("SES");
        }
        else{
        println!("INVALID INPUT");
        break;
      }
      rank.push(staff_rank);
    }

    else if staff_occu == "ACADEMIC"{

        let mut input4 = String::new();
        println!("\nStaff Occupation (Research assistant/PhD candidate/Post-doc researcher/senior lecturer/Dean): ");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let staff_rank = input4.trim().to_uppercase();

        if staff_rank == "RESEARCH ASSISTANT"{
            position.push("APS 3-5");
        }
        else if staff_rank == "PHD CANDIDATE"{
            position.push("APS 5-8");
        }
        else if staff_rank == "POST-DOC RESEARCHER"{
            position.push("EL1 8-10");
        }
        else if staff_rank == "SENIOR LECTURER"{
            position.push("EL2 10-13");
        }
        else if staff_rank == "DEAN"{
            position.push("SES");
        }
        else{
        println!("INVALID INPUT");
        break;
      }
      rank.push(staff_rank);
    }
    else if staff_occu == "LAWYER"{

        let mut input4 = String::new();
        println!("\nStaff Occupation (Paralegal/Junior associate/Senior associate(1/2/3/4)/Partner): ");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let staff_rank = input4.trim().to_uppercase();

        if staff_rank == "PARALEGAL"{
            position.push("APS 1-2");
        }
        else if staff_rank == "JUNIOR ASSOCIATE"{
            position.push("APS 3-5");
        }
        else if staff_rank == "ASSOCIATE"{
            position.push("APS 5-8");
        }
        else if staff_rank == "SENIOR ASSOCIATE 1"{
            position.push("EL1 8-10");
        }
        else if staff_rank == "SENIOR ASSOCIATE 2"{
            position.push("EL1 8-10");
        }
        else if staff_rank == "SENIOR ASSOCIATE 3"{
            position.push("EL2 10-13");
        }
        else if staff_rank == "SENIOR ASSOCIATE 4"{
            position.push("EL2 10-13");
        }
        else if staff_rank == "PARTNER"{
            position.push("SES");
        }
        else{
        println!("INVALID INPUT");
        break;
        }
        rank.push(staff_rank);
    }

    else if staff_occu == "TEACHER"{

        let mut input4 = String::new();
        println!("\nChoose from the following staff occupation 
            Placement
            Classroom Teacher
            Senior Teacher
            Leading Teacher
            Deput Principal
            Principal)");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let staff_rank = input4.trim().to_uppercase();

        if staff_rank == "Placement"{
            position.push("APS 1-2");
        }
        else if staff_rank == "Classroom Teacher"{
            position.push("APS 3-5");
        }
        else if staff_rank == "Senior Teacher"{
            position.push("APS 5-8");
        }
        else if staff_rank == "Leading Teacher"{
            position.push("EL1 8-10");
        }
        else if staff_rank == "Deput Principal"{
            position.push("EL2 10-13");
        }
        else if staff_rank == "Principal"{
            position.push("SES");
        }
        else{
        println!("INVALID INPUT");
        break;
        }
        rank.push(staff_rank);
    }

    occupation.push(staff_occu);
    name.push(staff_name);

}

    for x in 0..staff_num{
        println!("\n\n***STAFF No.{} INFORMATION***", x + 1);
        println!("\nNAME: {}", name[x]);
        println!("\nOCCUPATION: {}",occupation[x]);
        println!("\nRANK: {}",rank[x]);
        println!("\nSTAFF LEVEL: {}",position[x]);
    }
}
