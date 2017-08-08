
fn main(){
 print_even(10);
}

fn print_even(num:u32) -> u32{
 if num == 0{
  return num;
 }else{
  if num % 2 == 0{
   println!("{}", num);
  }
  return print_even(num - 1);
 }
}


