
fn main(){
 let mut arr_numbers: [i32; 3] = [1, 2, 3];
 let mut min = arr_numbers[0];

 for x in 0..arr_numbers.len(){
  if arr_numbers[x] < min{
   min = arr_numbers[x];
  }
 }

 println!("Lowest number: {}", min);
}


