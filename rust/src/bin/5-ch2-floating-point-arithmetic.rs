fn main() {
    println!("The sum is {}", 80.3 + 34.9);
    println!("{}", 600. / 7.);
    
    // you cannot mix float numbers with integer numbers, because the conversion in rust must be explicit
   // println!("{}", 2.7 + 1);
   
   // remainder with floatnumbers works but follows the same rule
   println!("{} {}", -12 % 10, -1.2 % 1.);
}
