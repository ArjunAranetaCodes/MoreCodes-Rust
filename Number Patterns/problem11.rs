/*
Problem 11: Write a program to print the pattern of asterisks using loop.
*
**
***
****
*****
*/
fn main(){
 for y in 0..6 {
  for x in 0..y {
   print!("*");
  }
  print!("\n");
 }
}