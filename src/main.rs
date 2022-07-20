use std::io;
use std::cmp::Ordering;
use std::io::Read;
use std::process::exit;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().to_lowercase().as_str() {
            "quit" | "exit" => {
                exit(0);
            },
            x => match x.parse() {
                Ok(num) => num,
                Err(_) => continue,
            },
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("Press ENTER to continue...");
                let _ = io::stdin().read(&mut [0u8]).unwrap();
                break;
            }
        }
    }
}
