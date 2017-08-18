
fn main(){
 let mut arr_numbers: [i32; 3] = [1, 2, 3];
 let mut indexOfNum = 0;

 for x in 1..arr_numbers.len()-1{
  if arr_numbers[x] == 2{
   indexOfNum = x
  }
 }
 println!("{} ", indexOfNum);
}


