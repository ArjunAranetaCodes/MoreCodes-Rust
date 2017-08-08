
fn main(){
 let mut sum = 0;

 for x in 1..31{
  if x % 5 == 0 {
   sum = sum + x;
  }
 }

 print!("Sum of numbers divisible by 5 from 1 to 30 is {}", sum);
}


