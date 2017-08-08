
fn main(){
 let num = 3333;
 let mut sum = 0;
 let mut temp = 0;
 let mut rmdr = 0;

 temp = num;

 while (temp != 0){
  rmdr = temp % 10;
  sum = sum * 10 + rmdr;
  temp = temp / 10;
 }

 if num == sum{
  print!("Palindrome number");
 }else{
  print!("Not a Palindrome number");
 }
}


