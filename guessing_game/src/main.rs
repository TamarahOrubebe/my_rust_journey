use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;


fn main() {
    println!("The guessing game!");

    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The sceret number is: {secret_number}");

    
    loop {
            println!("Please enter a number!");

           let mut guess = String::new();

            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too low!".red()),
                Ordering::Greater => println!("{}", "Too big!".red()),
                Ordering::Equal => {
                    println!("{}", "You win!".green());
                    break;
                }
            }

    }
   
    
}

