/*
Problem 18: Write a program to print the number pattern using loop.
  1  
 111
11111
 111
  1
*/
fn main(){
 let mut y = 0;
 let mut x = 0;
 let mut rows = 3;
 let mut ones = 1;
 let mut blank = rows - 1;

 for y in 1..(rows * 2) {
  for x in 0..blank {
   print!(" ");
  }
  
  for x in 1..(ones * 2) {
   print!("1");
  }
  
  print!("\n");
     
  if y < rows {
   blank -= 1;
   ones += 1;
  }
  else {
   blank += 1;
   ones -= 1;
  }
 }
}