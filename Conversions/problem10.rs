//Problem 10: Write a program that converts a decimal number to octal number.
fn main(){
 let mut dec = 256;
 let mut oct:String = "".to_string();

 while dec > 0{
  let con = format!("{}{}", dec % 8, oct);
  oct = con;
  dec = dec / 8;
 }

 println!("{}", oct);
}
