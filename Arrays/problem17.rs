
fn main(){
 let mut array1: [i32; 3] = [1, 2, 3];
 let mut array2: [i32; 4] = [0; 4];

 for x in 0..array1.len(){
  array2[x] = array1[x];
 }

 array2[array1.len()] = 10;

 for x in array2.iter() {
  println!("{}", x);
 }
}


