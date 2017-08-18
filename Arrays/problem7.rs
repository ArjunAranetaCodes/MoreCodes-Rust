
fn main(){
 let mut arr_numbers: [i32; 3] = [1, 2, 3];
 let mut max = 0;

 for x in 0..arr_numbers.len(){
  if arr_numbers[x] > max{
   max = arr_numbers[x];
  }
 }

 println!("Largest number: {}", max);
}


