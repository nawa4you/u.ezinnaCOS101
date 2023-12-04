fn main() {
   
   //name vector
   let name = Vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

   //age vector
   let age = Vec![16,17,19,22,20,21,18,23];

   print!("\nAge allocation:\n");

   //loop to iterate elements in vector
   for i in 0..age.len()
   {
        //iterating through i on the vector
        println!("{} is {} years old\n",name[i],age[i]);
   
   }


}