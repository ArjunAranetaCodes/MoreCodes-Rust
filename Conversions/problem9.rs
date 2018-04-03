//Problem 9: Write a program that converys a hexadecimal number to decimal number.
fn main(){
 let mut dec = 0;
 let hex = "100";
 let newhex:String = hex.chars().rev().collect();

 for x in 0..newhex.chars().count(){
  let slice = &newhex[x..(x+1)];
  let num = slice.parse::<i32>().unwrap();
  let num2 = x as u32;
  dec = dec + (num * (16_i32.pow(num2)));
 }

 println!("{:?}", dec);
}
