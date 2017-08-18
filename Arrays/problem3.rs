
fn main(){
 let mut arr_numbers: [i32; 4] = [1, 1, 2, 3];
 let mut count = 0;

 for x in 0..arr_numbers.len()-1{
  if arr_numbers[x] == 1{
   count = count + 1
  }
 }
 println!("Occurrence: {} ", count);
}


