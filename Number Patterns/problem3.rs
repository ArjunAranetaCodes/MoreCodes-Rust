/*
Problem 3: Write a program to print the number pattern of ones and zeros using loop.
01010
01010
01010
01010
01010
*/
fn main(){
 for y in 0..5 {
  for x in 0..5 {
   if x % 2 == 0 {
    print!("0")
   }
   else {
    print!("1")
   }
  }
  print!("\n")
 }
}