use rand::Rng;

pub fn rand() {

    // Generate a random number between 1 and 100
    let correct = rand::thread_rng().gen_range(1..101);
      println!("The correct answer is {}", correct);
      println!("Guess a number between 1 and 100");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Compare from the user
    let guess = guess.trim().parse::<u32>().expect("Please type a  Guessing number!");

  let mut message = String::new();

     // Guess the number
       if guess > correct {
           message = String::from("Your Guess is too high!");
       }else if guess < correct {
           message = String::from("Your Guess is too low!");
       }else {
           message = String::from("Your guess is correct!");
       }

  println!("{message}");

}