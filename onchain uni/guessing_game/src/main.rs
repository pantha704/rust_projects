use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Generating a random number between 1 and 100... \nDone!");

    loop {
        println!("Please input your guess.");
        // Create a new empty string to store the user's guess
        let mut guess = String::new();

        // Read the user's guess from the console
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert the user's guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // cmp compares two values and returns an Ordering (Less, Greater, or Equal).
        // guess.cmp(&secret_number) compares the user's guess with the secret number.
        match guess.cmp(&secret_number) {
            // If the guess is less than the secret number, print "Too small!"
            Ordering::Less => println!("Too small!"),
            // If the guess is greater than the secret number, print "Too big!"
            Ordering::Greater => println!("Too big!"),
            // If the guess is equal to the secret number, print "You win!" and exit the loop.
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
