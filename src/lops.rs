use rand::Rng;
use std::cmp::Ordering;
pub fn loop_str() {



    // Generate a random number between 1 and 100
    let correct = rand::thread_rng().gen_range(1..101);
    println!("The correct answer is {}", correct);
    println!("Guess a number between 1 and 100");


    // Loop guessing
    loop {

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Match Against Guess
      let guess= match guess.trim().parse::<u32>(){
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again! {}", e);
                continue;
            },

        };
        // Guess the number with loopstatement
        match guess.cmp(&correct) {
            std::cmp::Ordering::Less => println!("Your Guess is too low!"),
            std::cmp::Ordering::Greater => println!("Your Guess is too high!"),
            std::cmp::Ordering::Equal => {
                println!("Your guess is correct!");
                break;
            }
        };

    }

}