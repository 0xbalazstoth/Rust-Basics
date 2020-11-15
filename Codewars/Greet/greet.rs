use std::string;

fn greet(input : &str) -> String {
    if input == "Johnny" {
      return "Hello, my love!".to_string();
    }
    return format!("Hello, {}", input);
}

fn main() {
    println!("{}", greet("Johnny"));
}