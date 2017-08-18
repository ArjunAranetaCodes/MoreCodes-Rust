
fn main(){
 let mut arr_numbers: [i32; 4] = [4, 3, 2, 1];
 let mut closest = 0;
 let mut num_diff = arr_numbers[0];

 for x in 0..arr_numbers.len() {
  let mut diff = 0 - arr_numbers[x];
  diff = diff.abs();
  if diff < num_diff {
   num_diff = diff;
   closest = arr_numbers[x];
  }
 }

 println!("Closest to 0 is {}", closest);
}


