
fn main(){
 let mut arr_numbers: [i32; 4] = [1, 2, 3, 4];

 for x in arr_numbers.iter() {
  if x % 2 == 0{
   println!("{}", x);
  }
 }
}


