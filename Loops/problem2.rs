
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
 for x in 0..5{
  let secret_number = rand::thread_rng().gen_range(1, 9);
  println!("{}", secret_number);
 }
}


