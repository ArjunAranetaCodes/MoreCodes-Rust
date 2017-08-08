
fn main(){
 let mut num = 5;
 let mut fact = 1;

 for x in (1..num+1).rev(){
  fact = fact * x;
 }
 print!("Factorial of 5 is {}", fact);
}


