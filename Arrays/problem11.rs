
fn main(){
 let mut array1: [i32; 3] = [1, 2, 3];
 let mut array2: [i32; 3] = [0; 3];

 let mut y = array2.len();
 for x in 0..array1.len(){
  array2[x] = array1[y - 1];
  y = y - 1;
 }

 for x in 0..array2.len(){
  println!("{}", array2[x]);
 }
}


