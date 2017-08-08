
use std::io;

fn main() {
 println!("Enter the value of radius: ");
 let mut radius = String::new();
 io::stdin().read_line(&mut radius).ok().expect("failed to read radius");
 let radius: f32 = match radius.trim().parse() {
  Ok(num) => num,
  Err(_) => 0.0
 };

 let pi = 3.14;
 let dia = radius * radius;
 let area = pi * dia;
 let cir = 2.0 * pi * radius;

 print!("Diameter of the circle is {} \n", dia);
 print!("Area of the circle is {} \n", area);
 print!("Circumference of the circle is {} \n", cir);

}


