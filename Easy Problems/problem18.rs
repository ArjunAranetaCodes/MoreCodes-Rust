
fn main(){
 let word = "program";
 let letter = "a";
 let mut letter_count = 0;

 for x in 0..word.chars().count(){
  let slice = &word[x..(x+1)];
  if letter.contains(slice){
   letter_count = letter_count + 1;
  }
 }

 print!("Total: {}", letter_count);
}


