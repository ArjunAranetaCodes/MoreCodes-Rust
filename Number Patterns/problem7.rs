/*
Problem 7: Write a program to print the number pattern of ones and zeros using loop.
10101
01010
10101
01010
10101
*/
fn main(){
 let mut count = 0;
 for y in 0..5 {
  for x in 0..5 {
   count += 1;
   if (count % 2 == 1) {
    print!("1");
   }
   else{
    print!("0");
   }
  }
  print!("\n");
 }
}