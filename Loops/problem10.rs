
fn main(){
 let num = 371;
 let mut sum = 0;
 let mut temp = 0;
 let mut rmdr = 0;

 temp = num;

 while (temp != 0){
  rmdr = temp % 10;
  sum = sum + (rmdr * rmdr * rmdr);
  temp = temp / 10;
 }

 if num == sum{
  print!("Armstrong number");
 }else {
  print!("Not an Armstrong number");
 }
}


