extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!!! Welcome to ... Guess the number !!!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

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
