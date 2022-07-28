use std::io;

fn main() {
    println!("Let's play a guess game.");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guess is {guess}");
}