use std::io;

fn main() {
    // Prompt the user for their name
    println!("What is your name?");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    
    let name = name.trim();
    
    // Print a personalized greeting
    println!("Hello, {}! Welcome to the Rust project.", name);
}
