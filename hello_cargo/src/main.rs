use std::io;

fn main() {
    println!("Hello, world!");

    // User input from standard io.
    println!("Let's play guess the number!");
    println!("Please input your guess: ");
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
