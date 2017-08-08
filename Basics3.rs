//Variables and Input

use std::io;

fn main() {
 println!("What is your name? ");
 let mut name = String::new();
 io::stdin().read_line(&mut name).ok().expect("failed to read name");

 println!("What is your sex? (M or F) ");
 let mut sex = String::new();
 io::stdin().read_line(&mut sex).ok().expect("failed to read sex");

 println!("What is your age? ");
 let mut age = String::new();
 io::stdin().read_line(&mut age).ok().expect("failed to read age");
 let age: u32 = match age.trim().parse() {
  Ok(num) => num,
  Err(_) => 0
 };

 print!("Name: {}", name);
 print!("Sex: {}", sex);
 print!("Age: {}", age);
}



