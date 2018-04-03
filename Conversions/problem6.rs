//Problem 6: Write a program that converts a binary number to decimal number.
fn main(){
 let mut dec = 0;
 let binary = "110";
 let newbinary:String = binary.chars().rev().collect();

 for x in 0..newbinary.chars().count(){
  let slice = &newbinary[x..(x+1)];
  let num = slice.parse::<i32>().unwrap();
  dec = dec + (num * (2 ^ num));
 }

 println!("{:?}", dec);
}
