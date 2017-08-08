
fn main(){
 let word = "program";
 let vowels = "aeiou";
 let mut vowel_count = 0;

 for x in 0..word.chars().count(){
  let slice = &word[x..(x+1)];
  if vowels.contains(slice){
   vowel_count = vowel_count + 1;
  }
 }

 print!("Total Vowels: {}", vowel_count);
}


