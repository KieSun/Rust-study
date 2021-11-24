use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", number);
    println!("Please input you guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess)
}
