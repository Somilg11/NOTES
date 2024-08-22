use rand::Rng;
use std::io;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() { //shadowing
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
        }
    }
}
