
fn main(){
 let mut array1: [i32; 4] = [1, 3, 2, 4];
 let mut array2: [i32; 4] = [0; 4];
 array1.sort();

 let mut y = array1.len();
 for x in 0..array1.len(){
  array2[x] = array1[y - 1];
  y = y - 1;
 }

 for x in array2.iter() {
  println!("{}", x);
 }
}


