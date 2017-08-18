
fn main(){
 let mut array1: [i32; 3] = [1, 2, 3];
 let mut array2: [i32; 3] = [0; 3];
 array2 = array1;

 for x in array2.iter() {
  println!("{}", x);
 }
}


