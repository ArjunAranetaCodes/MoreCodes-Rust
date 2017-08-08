
use std::io;

fn main(){
 let mut sum = 0;
 let mut ave = 0;

 for x in 0..5 {
  let mut num = String::new();
  println!("Enter a number: ");
  io::stdin().read_line(&mut num).ok().expect("failed to read num");
  let num: u32 = match num.trim().parse() {
   Ok(num) => num,
   Err(_) => 0
  };

  sum = sum + num;
 }
 ave = sum / 5;
 print!("Average is {}", ave);
}


