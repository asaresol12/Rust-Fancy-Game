use rand::Rng;

pub fn match_str() {

    // Generate a random number between 1 and 100
    let correct = rand::thread_rng().gen_range(1..101);
    println!("The correct answer is {}", correct);
    println!("Guess a number between 1 and 100");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Compare from the user
    let guess = guess.trim().parse::<u32>().expect("Please type a  Guessing number!");

    // Guess the number with Match

       let message = match guess.cmp(&correct) {
           std::cmp::Ordering::Less => "Your Guess is too low!",
           std::cmp::Ordering::Greater => "Your Guess is too high!",
           std::cmp::Ordering::Equal => "Your guess is correct!",
       };

         println!("{message}");



}