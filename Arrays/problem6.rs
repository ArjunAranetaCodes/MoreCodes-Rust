
fn main(){
 let mut arr_numbers: [i32; 3] = [1, 2, 3];
 let mut count = 0;

 for x in 0..arr_numbers.len(){
  if arr_numbers[x] == 2{
   count = count + 1
  }
 }

 if count > 0 {
  println!("Contains 2");
 }else{
  println!("Does not contains 2");
 }

}


