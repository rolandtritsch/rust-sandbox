extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;

fn main() {
    let args: Vec<_> = env::args().collect();
    let cmd = args[0].to_string();
    let max: u32;
    match args.len() {
        2 => {
            max = args[1].to_string().trim().parse().ok().expect("Failed to parse <max>");
        }
        _ => {
            println!("Usage: {} <max>", cmd);
            return;
        }
    }

    println!("!!! Welcome to ... Guess the number !!!");
    let secret_number = rand::thread_rng().gen_range(1, max + 1);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).ok().expect("Failed to read number");
        let guess: u32 = guess.trim().parse().ok().expect("Failed to parse number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That's it!!!");
                break;
            }
        }
    }
}
