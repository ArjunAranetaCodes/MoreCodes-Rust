
use std::io;

fn main(){
 loop {
  let mut num = String::new();
  println!("Enter a number: ");
  io::stdin().read_line(&mut num).ok().expect("failed to read num");
  let num2: i32 = match num.trim().parse() {
   Ok(num2) => num2,
   Err(_) => 0
  };
  if num2 < 0 {
   break;
  }
 }
 print!("Terminated");
}


