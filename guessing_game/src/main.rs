extern crate rand;

use std::io::stdin;
use std::cmp::Ordering::*;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Turbo-fish!
        let guess = match guess.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
