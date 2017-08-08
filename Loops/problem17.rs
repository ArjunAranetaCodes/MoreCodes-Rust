
fn main(){
 let mut num1 = 1;
 let mut num2 = 1;

 println!("{}",num1);
 while num2 < 100 {
  println!("{}", num2);
  num2 = num2 + num1;
  num1 = num2 - num1;
 }
}


