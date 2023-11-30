use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main () {
    println!("Welcome to the guessing game where you guess the secret number");

    loop {
        println!("Please input your number");

        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is {}", &secret_number);

        io::stdin()
            .read_line(&mut guess).expect("Error reading input");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("The number must be between 1 and 100");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => continue
        };

       

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low!".red()),
            Ordering::Greater => println!("{}", "Too high!".red()),
            Ordering::Equal =>  {
                println!("{}", "You win!".green());
                break;
            }
            
        }
    }
   

}