/*
Problem 8: Write a program to print the number pattern of ones and zeros using loop.
11111
11111
11011
11111
11111
*/
fn main(){
 let mut row = 5;
 let mut col = 5;
 for y in 0..row { 
  for x in 0..col {
   if (x == (row / 2) && y == (col / 2)) {
    print!("0");
   }
   else {
    print!("1");
   }
  }
  print!("\n");
 }
}