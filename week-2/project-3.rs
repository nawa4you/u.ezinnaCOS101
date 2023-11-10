 
fn main () {
        let p = 210_000;
        let r = 5;
        let n = 3;
        let z:f64 = 0.05;
        let w:f64 = 1 - z;
        let y = w ^ n;
        println!("z = {}", z);
        println!("y = {}", y);
        println!("w = {}", w);
        let a = p * ((1 - (r / 100)) ^ n);
        println!("Let the value of the TV after 3 years = A");
        println!("From the equation A = P[1 - (R / 100)]^n");
        println!("We can find how much the price of the TV depreciated after 3 y>
        println!("Brings us down to A = {} X [1 - ( {} / 100 )]^ {}", p, r, n);
        println!("Therefore A = N{}", a);
        println!("Solution may not be accurate due to no implementation of f64^f>
}



