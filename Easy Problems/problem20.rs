
fn main(){
 let word = "anna";
 let temp_word:String = word.chars().rev().collect();

 if word == temp_word {
  print!("Palindrome");
 }else{
  print!("Not Palindrome");
 }
}


