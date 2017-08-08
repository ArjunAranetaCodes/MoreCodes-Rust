
use std::io;

fn main() {
 println!("Enter the value of num: ");
 let mut num = String::new();
 io::stdin().read_line(&mut num).ok().expect("failed to read num");
 let num: u32 = match num.trim().parse() {
  Ok(num) => num,
  Err(_) => 0
 };

 if num % 2 == 0 {
  print!("Number is even");
 }else{
  print!("Number is odd");
 }
}

