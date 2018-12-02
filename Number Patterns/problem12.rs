/*
Problem 12: Write a program to print the pattern of asterisks using loop.
*****
****
***
**
*
*/
fn main(){
 let mut row = 6;
 for y in (0..row).rev() {
  for x in 0..y {
   print!("*");
  }
  print!("\n");
 }
}