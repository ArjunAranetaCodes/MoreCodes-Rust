
use std::io;

fn main() {
 println!("Enter the value of celsius: ");
 let mut celsius = String::new();
 io::stdin().read_line(&mut celsius).ok().expect("failed to read celsius");
 let celsius: f32 = match celsius.trim().parse() {
  Ok(num) => num,
  Err(_) => 0.0
 };

 let farenheit = (9.0 / 5.0) * celsius + 32.0;

 print!("{}C equals to {}F \n", celsius, farenheit);

}


