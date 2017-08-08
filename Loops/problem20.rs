
fn main(){
 print!("Sum is {}", get_sum(10,0));
}

fn get_sum(num: u32, sum: u32) -> u32{
 if num == 0{
  return sum;
 }else{
  return get_sum((num - 1), (sum + num));
 }
}


