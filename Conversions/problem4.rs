//Problem 4: Write a program that converts a string to array/list.
fn main(){
 let numbers = "1,2,3";
 let arrnumbers = numbers.split(",");

 for num in arrnumbers{
  println!("{}", num);
 }
}
