use std::io;

pub fn guess() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess_num = String::new();

    io::stdin()
        .read_line(&mut guess_num)
        .expect("Failed to read line");

    println!("You guessed: {}", guess_num);
}
