
fn main(){
 let mut array1: [i32; 4] = [1, 2, 3, 4];
 let mut n = 2;
 let mut temp_num = 0;
 let mut index_num = 0;

 for x in 0..array1.len(){
  if array1[x] == n {
   index_num = x;
  }
 }

 for x in 0..array1.len(){
  if (x >= index_num) && (x < array1.len() - 1){
   array1[x] = array1[x + 1];
  }
 }

 let mut y = array1.len() - 1;
 array1[y] = 0;

 for x in array1.iter(){
  println!("{}", x);
 }
}


