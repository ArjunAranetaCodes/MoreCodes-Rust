
use std::io;

fn main() {
 println!("Enter the value of num1: ");
 let mut num1 = String::new();
 io::stdin().read_line(&mut num1).ok().expect("failed to read num1");
 let num1: u32 = match num1.trim().parse() {
  Ok(num) => num,
  Err(_) => 0
 };

 println!("Enter the value of num2: ");
  let mut num2 = String::new();
  io::stdin().read_line(&mut num2).ok().expect("failed to read num2");
  let num2: u32 = match num2.trim().parse() {
   Ok(num) => num,
   Err(_) => 0
 };

 let sum = num1 + num2;

 print!("Sum is {}", sum);
}


