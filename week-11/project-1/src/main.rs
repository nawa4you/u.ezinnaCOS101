
use std::io;
use std::io::Write;
fn main() {

       //create struc and vec items
       struct Company {
           name: String,
           shares: f32,
           liabilities: f32,
           year: u16,
       }



       impl Company {
          fn leverage(&self) -> f64 {
             ((self.shares - self.liabilities)/self.liabilities * 100.0).into()
                }

          fn fivepctlvrg(&self) -> f64 {
             (0.05 * self.leverage()).into()
          }
          
       }




       //vector deffinition
          let companies = vec![
                  Company {
                  name:String::from("Cadbury Nigeria Plc"),
                  shares:15_000_000.0,
                  liabilities:5_500_000.0,
                  year:1965
                  },
                  Company {
                  name:String::from("Champion Brewries Plc"),
                  shares:25_000_000.0,
                  liabilities:8_000_000.0,
                  year:1974
                  },
          
                  Company {
                  name:String::from("Dangote Sugar Refinery Plc"),
                  shares:18_000_000.0,
                  liabilities:10_000_000.0,
                  year:1970
                  },
                  Company {
                  name:String::from("Flour Mills Nigeria Plc"),
                  shares:32_000_000.0,
                  liabilities:4_000_000.0,
                  year:1960
                  },
                  Company {
                  name:String::from("Nestle Nigeria Plc"),
                  shares:8_000_000.0,
                  liabilities:1_500_000.0,
                  year:1961
                  },
                  Company {
                  name:String::from("Unilever Nigeria Plc"),
                  shares:37_000_000.0,
                  liabilities:11_000_000.0,
                  year: 1923
                  },
                  Company {
                  name:String::from("Honeywell Nigeria Plc"),
                  shares:34_000_000.0,
                  liabilities:9_000_000.0,
                  year:1906
                  },
                  Company {
                  name:String::from("Nigerian Brewries Plc"),
                  shares:30_000_000.0,
                  liabilities:12_000_000.0,
                  year:1946
                  },
                ];

       let sn = vec!["1","2","3","4","5","6","7","8"];
       let usernames = vec!["cadb","cham","dang","flou","nest","unil","hone","nige"];



        // Iterate the vector and access each struct item to a personal vec 
        let mut years: Vec<i32> = Vec::new();
        for comp in &companies {
            years.push(comp.year.into());
        } 

        let mut cname: Vec<&str> = Vec::new();
        for comp in &companies {
            cname.push(&comp.name);
        } 

        let mut share: Vec<f32> = Vec::new();
        for comp in &companies {
            share.push(comp.liabilities);
        }

        let mut liab: Vec<f32> = Vec::new();
        for comp in &companies {
            liab.push(comp.liabilities);
        }

       //ASK USER LOGIN

       println!("***COMPANY DATA OF ASSETS AND LIABILITIES BY SPRINGATE MODEL***");
     
                println!("Please enter your username and password to view:");
                loop{
                    let mut input1 = String::new();
                   println!("Username: ");
                    io::stdin().read_line(&mut input1).expect("Invalid input");
                    let username: &str = input1.trim();
                    //check if username is stored
                    if usernames.contains(&username) {
                          break;
                        } else {
                            println!("Username not found, Please try again(using the first 4 letters in company name. ie cadb, cham, dang, flou, nest, unil, hone or nige)");
                        }
            
                }
                loop { 
                   let mut input2 = String::new();
                   println!("Password: ");
                   io::stdin().read_line(&mut input2).expect("Invalid input");
                   let password = input2.trim();
                      if password.chars().any(|c| c.is_ascii_uppercase()) || password.contains(|c| c == '$' || c == '#' || c == '@') {       
                         println!("Please password should not contain capital letters, $, #, or @.");
                      }
                   if password.len() >= 0 && password.len() <= 9 {
                            break;
                      } 


                      else {
                        println!("Please password should be between 0-9 characters. Try again");
                   }

                }

                
        //writing file
        let mut all_comp_file = std::fs::File::create("All Company Data.txt").expect("Cant make file");


        writeln!(all_comp_file,"Here is the data for all companiies year of establishment, company name, assets, liabilities:  ");
        writeln!(all_comp_file,"{:>10}| {:>10}| {:>25}| {:>25}| {:>25}|", "S/N", "Year","Company Name", "Company Assets", "Company liability" );

        for i in 0..sn.len() {
        writeln!(all_comp_file,"{:>10}| {:>10}| {:>20}| {:>20}| {:>20}| {:>20}", sn[i], years[i], cname[i], share[i], liab[i],companies[i].leverage() );

        }

       //telling user file has been made
        println!("Data for all companies written Succesfully to file (@ All Company Data.txt)");

            loop {

                let mut whichcomp = String::new();
                println!("Alright, now which company are you representing?\n\n1 {}\n2 {}\n3 {}\n4 {}\n5 {}\n6 {}\n7 {}\n8 {}", companies[0].name,companies[1].name,companies[2].name,companies[3].name,companies[4].name,companies[5].name,companies[6].name,companies[7].name);
                io::stdin().read_line(&mut whichcomp).expect("Invalid input");
                let choice: i32 = whichcomp.trim().parse().expect("Invalid number");
             if choice == 1 {
             if let Some(company) = companies.get(0) {
               println!("Hello user from {}", company.name);
               let mut special10mil = std::fs::File::create("Companies Liabilities Below 10,000,000.txt").expect("Cant make file");
   println!("Your companies liabilities is less than 10,000,000.\nAs a bonus to the original file, creating 5% of percentage leverage used by each company in another file....");

   writeln!(special10mil, "Here is companies with 10m liabilities and below");
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}",  "S/N","Companies Name", "Companies Leverage");

   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "1",companies[0].name,format!("{:.2}", companies[0].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "2",companies[1].name, format!("{:.2}",companies[1].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "3",companies[3].name, format!("{:.2}",companies[3].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "4",companies[4].name, format!("{:.2}",companies[4].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "5",companies[6].name, format!("{:.2}",companies[6].fivepctlvrg()));
   writeln!(special10mil, "Thats all companies with 10m liabilities and below");
                }
                break;
             }   else if choice == 2 {
              if let Some(company) = companies.get(1) {
                  println!("Hello user from {}", company.name);
                      let mut special20mil = std::fs::File::create("Companies Shares Above 20,000,000.txt").expect("Cant make file");
   println!("Your companies share is greater than 20,000,000.\nAs a bonus to the original file, creating the percentage leverage used by each company in another file....");

   println!("Done Creating file for companieswith 20,000,000 in shaeres and above");  
   writeln!(special20mil, "Here is companies with 20m shares and above");
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}",  "S/N","Companies Name", "Companies Leverage");

   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "1",cname[1], format!("{:.2}",companies[1].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "2",companies[3].name, format!("{:.2}",companies[3].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "3",companies[5].name, format!("{:.2}",companies[5].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "4",companies[6].name, format!("{:.2}",companies[6].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "5",companies[7].name, format!("{:.2}",companies[7].leverage()));
   writeln!(special20mil, "Thats all companies with 20m shares and above");

   println!("Done Creating file for companieswith 10,000,000 in liabilities and below");
                  let mut special10mil = std::fs::File::create("Companies Liabilities Below 10,000,000.txt").expect("Cant make file");
   println!("Your companies liabilities is less than 10,000,000.\nAs a bonus to the original file, creating 5% of percentage leverage used by each company in another file....");

   writeln!(special10mil, "Here is companies with 10m liabilities and below");
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}",  "S/N","Companies Name", "Companies Leverage");

   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "1",companies[0].name,format!("{:.2}", companies[0].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "2",companies[1].name,format!("{:.2}", companies[1].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "3",companies[3].name,format!("{:.2}", companies[3].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "4",companies[4].name,format!("{:.2}", companies[4].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "5",companies[6].name,format!("{:.2}", companies[6].fivepctlvrg()));
   writeln!(special10mil, "Thats all companies with 10m liabilities and below");

              }
              break;
          } else if choice == 3 {
              if let Some(company) = companies.get(2) {
                  println!("Hello user from {}", company.name);
              }
              break;
          } else if choice == 4 {
              if let Some(company) = companies.get(3) {
                  println!("Hello user from {}", company.name);
                      let mut special20mil = std::fs::File::create("Companies Shares Above 20,000,000.txt").expect("Cant make file");
   println!("Your companies share is greater than 20,000,000.\nAs a bonus to the original file, creating the percentage leverage used by each company in another file....");

   println!("Done Creating file for companieswith 20,000,000 in shaeres and above");  
   writeln!(special20mil, "Here is companies with 20m shares and above");
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "S/N","Companies Name", "Companies Leverage");

   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "1",cname[1],format!("{:.2}", companies[1].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "2",cname[3],format!("{:.2}", companies[3].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "3",cname[5],format!("{:.2}", companies[5].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "4",cname[6],format!("{:.2}", companies[6].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "5",cname[7],format!("{:.2}", companies[7].leverage()));
   writeln!(special20mil, "Thats all companies with 20m shares and above");

   println!("Done Creating file for companieswith 10,000,000 in liabilities and below");
                  let mut special10mil = std::fs::File::create("Companies Liabilities Below 10,000,000.txt").expect("Cant make file");
   println!("Your companies liabilities is less than 10,000,000.\nAs a bonus to the original file, creating 5% of percentage leverage used by each company in another file....");

   writeln!(special10mil, "Here is companies with 10m liabilities and below");
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}",  "S/N","Companies Name", "Companies Leverage");

   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "1",cname[0],format!("{:.2}", companies[0].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "2",cname[1],format!("{:.2}", companies[1].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "3",cname[3],format!("{:.2}", companies[3].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "4",cname[4],format!("{:.2}", companies[4].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "5",cname[6],format!("{:.2}", companies[6].fivepctlvrg()));
   writeln!(special10mil, "Thats all companies with 10m liabilities and below");
              }
              break;
          } else if choice == 5 {
              if let Some(company) = companies.get(4) {
                  println!("Hello user from {}", company.name);
                  let mut special10mil = std::fs::File::create("Companies Liabilities Below 10,000,000.txt").expect("Cant make file");
   println!("Your companies liabilities is less than 10,000,000.\nAs a bonus to the original file, creating 5% of percentage leverage used by each company in another file....");

   writeln!(special10mil, "Here is companies with 10m liabilities and below");
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "S/N","Companies Name", "Companies Leverage");

   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "1",companies[0].name, format!("{:.2}",companies[0].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "2",companies[1].name, format!("{:.2}",companies[1].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "3",companies[3].name, format!("{:.2}",companies[3].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "4",companies[4].name, format!("{:.2}",companies[4].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "5",companies[6].name, format!("{:.2}",companies[6].fivepctlvrg()));
   writeln!(special10mil, "Thats all companies with 10m liabilities and below");
              }
              break;
          } else if choice == 6 {
              if let Some(company) = companies.get(5) {
                  println!("Hello user from {}", company.name);
                      let mut special20mil = std::fs::File::create("Companies Shares Above 20,000,000.txt").expect("Cant make file");
   println!("Your companies share is greater than 20,000,000.\nAs a bonus to the original file, creating the percentage leverage used by each company in another file....");

   println!("Done Creating file for companieswith 20,000,000 in shaeres and above");  
   writeln!(special20mil, "Here is companies with 20m shares and above");
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "S/N","Companies Name", "Companies Leverage");

   writeln!(special20mil, "{:>20}| {:>20} | {:>20}",  "1",companies[1].name,format!("{:.2}", companies[1].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}",  "2",companies[3].name,format!("{:.2}", companies[3].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}",  "3",companies[5].name,format!("{:.2}", companies[5].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}",  "4",companies[6].name,format!("{:.2}", companies[6].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}",  "5",companies[7].name,format!("{:.2}", companies[7].leverage()));
   writeln!(special20mil, "Thats all companies with 20m shares and above");

   println!("Done Creating file for companieswith 10,000,000 in liabilities and below");
              }
              break;
          } else if choice == 7 {
              if let Some(company) = companies.get(6) {
                  println!("Hello user from {}", company.name);
                      let mut special20mil = std::fs::File::create("Companies Shares Above 20,000,000.txt").expect("Cant make file");
   println!("Your companies share is greater than 20,000,000.\nAs a bonus to the original file, creating the percentage leverage used by each company in another file....");

println!("Done Creating file for companies with 20,000,000 in shares and above");  
writeln!(special20mil, "Here are companies with 20m shares and above");
writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "S/N", "Companies Name", "Companies Leverage");

// Format leverage with two decimal places using format!
writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "1", companies[1].name, format!("{:.2}", companies[1].leverage()));
writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "2", companies[3].name, format!("{:.2}", companies[3].leverage()));
writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "3", companies[5].name, format!("{:.2}", companies[5].leverage()));
writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "4", companies[6].name, format!("{:.2}", companies[6].leverage()));
writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "5", companies[7].name, format!("{:.2}", companies[7].leverage()));
writeln!(special20mil, "That's all companies with 20m shares and above");


   println!("Done Creating file for companieswith 10,000,000 in liabilities and below");
                  let mut special10mil = std::fs::File::create("Companies Liabilities Below 10,000,000.txt").expect("Cant make file");
   println!("Your companies liabilities is less than 10,000,000.\nAs a bonus to the original file, creating 5% of percentage leverage used by each company in another file....");

   writeln!(special10mil, "Here is companies with 10m liabilities and below");
   writeln!(special10mil, "{:>20}| {:>25} | {:>20}", "S/N","Companies Name", "Companies 5% of Leverage");

   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "1",companies[0].name,format!("{:.2}", companies[0].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "2",companies[1].name,format!("{:.2}", companies[1].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "3",companies[3].name,format!("{:.2}", companies[3].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "4",companies[4].name,format!("{:.2}", companies[4].fivepctlvrg()));
   writeln!(special10mil, "{:>20}| {:>20} | {:>20}", "5",companies[6].name,format!("{:.2}", companies[6].fivepctlvrg()));
   writeln!(special10mil, "Thats all companies with 10m liabilities and below");
              }
              break;
          } else if choice == 8 {
              if let Some(company) = companies.get(7) {
                  println!("Hello user from {}", company.name);
                      let mut special20mil = std::fs::File::create("Companies Shares Above 20,000,000.txt").expect("Cant make file");
   println!("Your companies share is greater than 20,000,000.\nAs a bonus to the original file, creating the percentage leverage used by each company in another file....");

   println!("Done Creating file for companieswith 20,000,000 in shaeres and above");  
   writeln!(special20mil, "Here is companies with 20m shares and above");
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "S/N","Companies Name", "Companies Leverage");

   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "1",companies[1].name, format!("{:.2}",companies[1].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "2",companies[3].name, format!("{:.2}",companies[3].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "3",companies[5].name, format!("{:.2}",companies[5].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "4",companies[6].name, format!("{:.2}",companies[6].leverage()));
   writeln!(special20mil, "{:>20}| {:>20} | {:>20}", "5",companies[7].name, format!("{:.2}",companies[7].leverage()));
   writeln!(special20mil, "Thats all companies with 20m shares and above");

   println!("Done Creating file for companieswith 10,000,000 in liabilities and below");
              } 
                    break;
          }   else {
                  println!("Please select with a valid number");
              }
          }



}



