/*
Problem 16: Write a program to print the number pattern using loop.
12345
1234
123
12
1
*/
fn main(){
 let mut row = 6;
 for y in (0..row).rev() {
  for x in 0..y {
   print!("{}", (x + 1));
  }
  print!("\n");
 }
}