
fn main(){
 for x in 0..12{
  println!("{}", fibonacci(x));
 }
}
fn fibonacci(num:u32) -> u32{
 if (num == 1) || (num == 0){
  return num;
 }else{
  return (fibonacci(num - 1) + fibonacci(num - 2));
 }
}


