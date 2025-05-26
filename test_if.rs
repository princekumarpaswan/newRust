pub mod test_if{
    use std::io;

  pub fn test_if() -> String {
    let age = 16u8;
    println!("Enter your age");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let user_age: u8 = input.trim().parse().expect("Please enter a valid number");
    if user_age >= age {
       return ("you can drive a car").to_string();
    }else{
      return ( "you can not drive a car").to_string()
    }
}}


