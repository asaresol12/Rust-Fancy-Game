
pub fn guess() {


     println!("What is your name?");
    let mut name = String::new();
    // User Input
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    // Print Output
    println!("Hello {} Welcome!", name.trim());


}