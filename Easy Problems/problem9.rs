
use std::io;
fn main(){
 let mut sum = 0;

 println!("Enter the value of num: ");
 let mut num = String::new();
 io::stdin().read_line(&mut num).ok().expect("failed to read num");
 let num: u32 = match num.trim().parse() {
  Ok(num) => num,
  Err(_) => 0
 };

 for x in 1..(num + 1){
  sum = sum + x;
 }

 print!("Sum of 1 to {} is {}", num, sum);
}


