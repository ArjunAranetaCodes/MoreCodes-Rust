
use std::io;

fn main() {
 println!("Enter the value of length: ");
 let mut length = String::new();
 io::stdin().read_line(&mut length).ok().expect("failed to read length");
 let length: u32 = match length.trim().parse() {
  Ok(num) => num,
  Err(_) => 0
 };

 println!("Enter the value of width: ");
  let mut width = String::new();
  io::stdin().read_line(&mut width).ok().expect("failed to read width");
  let width: u32 = match width.trim().parse() {
   Ok(num) => num,
   Err(_) => 0
 };

 let area = length * width;

 print!("Area of Rectangle is {}", area);
}


