//Problem 8: Write a program that converts a decimal number to hexadecimal number.
fn main(){
 let mut dec = 256;
 let mut hex:String = "".to_string();

 while dec > 0{
  let con = format!("{}{}", dec % 16, hex);
  hex = con;
  dec = dec / 16;
 }

 println!("{}", hex);
}
