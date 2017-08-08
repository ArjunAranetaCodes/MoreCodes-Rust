//Loop Structures

fn main(){
 let mut x = 0;

 println!("Loop");
 loop{
  print!("{}", x);
  x += 1;

  if x == 11 {
   break;
  }
 }

 println!("\nWhile Loop");
 x = 0;
 while x <= 10{
  print!("{}", x);
  x += 1;
 }

 println!("\nFor Loop");
 for x in 0..11{
  print!("{}", x);
 }
}



