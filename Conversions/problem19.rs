//Problem 19: Write a program that converts numbers to words.
fn numbertowords(number: u32, mut word: &str) -> &str{
 let mut newword: &str = "";
 let mut newnum = number as usize;
 if (number >= 1) && (number <= 19){
  let map: [&str; 19] = ["One", "Two", "Three", "Four", "Five", "Six", "Seven",
    "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen",
    "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
  newword =  map[newnum - 1];
  word = newword;
 }

 return word;
}

fn main(){
 println!("{}", numbertowords(1, ""));
}
