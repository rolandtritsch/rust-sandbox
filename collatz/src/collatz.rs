#![allow(unused_assignments)]
#![allow(unused_parens)]

use std::env;

fn main() {
    let mut number = 0;
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => {number = args[1].parse::<u32>().unwrap();}
        _ => {println!("Usage: collatz <number>"); return}
    }
    println!("Collatz({}) -> {}", number, collatz(number));
}

fn collatz(n: u32) -> u32 {
    match n {
        1 => {0}
        _ => {
            if(n % 2 == 0) {1+collatz(n/2)}
            else {1+collatz(3*n+1)}
        }
    }
}
