
fn main(){
 let word = "MoreCodes";
 let mut new_word = "".to_string();
 for x in (0..(word.chars().count()-1)).rev(){
  let slice = &word[x..(x+1)].to_string();
  new_word.push_str(slice);
 }

 print!("{}", new_word);
}


