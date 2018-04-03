//Problem 5: Write a program that converts an array/list to string.
fn main(){
 let mut arr_numbers: [i32; 3] = [1, 2, 3];
 let mut numbers = "".to_string();

 for x in 0..arr_numbers.len(){
  numbers.push_str(&arr_numbers[x].to_string());
 }

 println!("{} ", numbers);
}
