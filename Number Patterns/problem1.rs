/*
Problem 1: Write a program to print the number pattern of ones and zeros using loop.
11111
11111
11111
11111
11111
*/
fn main(){
 for y in 0..5{
  for x in 0..5 {
   print!("1");
  }
  print!("\n");
 }
}