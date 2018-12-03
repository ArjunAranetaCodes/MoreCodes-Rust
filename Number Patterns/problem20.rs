/*
Problem 20: Write a program to print the number pattern using loop.
  1
 222
33333
 444
  5
*/
fn main(){
 let mut y = 0;
 let mut x = 0;
 let mut rows = 3;
 let mut nums = 1;
 let mut blank = rows - 1;

 for y in 1..(rows * 2) {
  for x in 0..blank {
   print!(" ");
  }
  
  for x in 1..(nums * 2) {
   print!("{}", y);
  }
  
  print!("\n");
     
  if y < rows {
   blank -= 1;
   nums += 1;
  }
  else {
   blank += 1;
   nums -= 1;
  }
 }
}