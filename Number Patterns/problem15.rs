/*
Problem 15: Write a program to print the number pattern using loop.
11111
2222
333
44
5
*/
fn main(){
 let mut row = 6;
 let mut num = 1;
 for y in 1..row {
  for x in (y..row).rev() {
   print!("{}", num);
  }
  num += 1;
  print!("\n");
 }
}