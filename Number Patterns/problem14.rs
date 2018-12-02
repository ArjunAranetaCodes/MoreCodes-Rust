/*
Problem 14: Write a program to print the number pattern using loop.
1
22
333
4444
55555
*/
fn main(){
 let mut row = 6;
 for y in 0..row {
  for x in 0..y {
   print!("{}", y);
  }
  print!("\n");
 }
}