//Problem 7: Write a program that converts a decimal number to binary number.
fn main(){
 let mut dec = 10;
 let mut binary:String = "".to_string();

 while dec > 0{
  let con = format!("{}{}", dec % 2, binary);
  binary = con;
  dec = dec / 2;
 }

 println!("{}", binary);
}
