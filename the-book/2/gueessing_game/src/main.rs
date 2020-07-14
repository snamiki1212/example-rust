extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("secret number is :{}.", secret_number);
    println!("plz input your guess num...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);
}
