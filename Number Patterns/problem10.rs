/*
Problem 10: Write a program to print the number pattern using loop.
12345
23456
34567
45678
56789
*/
fn main(){
 let mut row = 5;
 let mut col = 5;
 let mut min = 1;
 for y in 0..row  {
  let mut num = min + y;
  for x in 0..col {
   print!("{}", num);
   num = num + 1;
  }
  print!("\n");
 }
}