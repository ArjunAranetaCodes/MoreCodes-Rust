//Basics of Functions

fn function1(){
 println!("Hello there");
}

fn function2(num1: u32){
 println!("That number is {}", num1);
}

fn function3() -> u32{
 let sum = 1 + 1;
 return sum;
}

fn function4(first_name : &str, last_name : &str) -> String{
 let mut full_name : String = first_name.to_owned();
 full_name.push_str(" ");
 full_name.push_str(last_name);
 return full_name;
}

fn main(){
 function1();
 function2(5);
 println!("It's true! 1 + 1 = {}", function3());
 println!("Hello {}", function4("More", "Codes"));
}

