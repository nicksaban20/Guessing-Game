use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    
    let min = 1;
    let max = 50;
    let max_guesses = 5;

    println!("Guess the number between {min} and {max}!");

    let secret_number = rand::thread_rng().gen_range(min..=max);


    for n in 1..=max_guesses {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You won in {n} guesses!");
                break;
            }
        }


    }

}
