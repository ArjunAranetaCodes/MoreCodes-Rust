//Problem 20: Write a program that converts a number to its 
//corresponding month (e.g. 1 = January).
fn main(){
 let map: [&str; 12] = ["January", "February", "March", "April",
  "May", "June", "July", "August", "September", "October",
  "November", "December"];
 let num = 1 as usize;
 println!("{}", map[num - 1]);
}
