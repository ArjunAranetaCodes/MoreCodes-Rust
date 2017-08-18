
fn main(){
 let mut arr_numbers: [i32; 3] = [1, 2, 3];
 let mut sum = 0;
 let mut ave = 0;

 for x in 0..arr_numbers.len(){
  sum = sum + arr_numbers[x];
 }

 ave = sum / 3;

 println!("Average is: {}", ave);
}


