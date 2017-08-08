
fn main(){
 let word = "MoreCodes";
 let mut letter_count = 0;

 for letter in word.chars(){
  letter_count = letter_count + 1;
 }

 print!("String Length: {}", letter_count);
}


