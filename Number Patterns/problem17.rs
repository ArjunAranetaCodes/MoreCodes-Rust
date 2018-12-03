/*
Problem 17: Write a program to print the number pattern using loop.
1
12
123
1234
12345
1234
123
12
1
*/
fn main(){
 let mut row = 6;
 for y in 0..row {
  for x in 0..y {
   print!("{}", (x + 1));
  }
  print!("\n");
 }
 
 for y in (0..row - 1).rev() {
  for x in 0..y {
   print!("{}", (x + 1));
  }
  print!("\n");
 }
}