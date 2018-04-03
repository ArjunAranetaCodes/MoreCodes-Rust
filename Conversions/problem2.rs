//Problem 2: Write a program that converts a string to integer.
fn main(){
 let strnum = "10";
 let num = strnum.parse::<i32>().unwrap();
 println!("{:?}", num);
}
