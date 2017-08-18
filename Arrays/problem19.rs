
fn main(){
 let mut array1: [i32; 3] = [1, 2, 3];
 let mut array2: [i32; 3] = [1, 2, 3];
 let new_array = [&array1[..], &array2[..]].concat();

 for x in new_array.iter() {
  println!("{}", x);
 }
}


