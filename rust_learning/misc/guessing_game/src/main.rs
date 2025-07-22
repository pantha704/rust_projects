use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_number:u32 = rand::rng().random_range(1..=108);
    // println!("Guess is {}",guess);
    
    loop {
    let mut guess = String::new();
    println!("\nEnter your guess");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
 
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
     }
    }
}
