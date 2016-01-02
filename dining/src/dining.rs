#![allow(dead_code)]

extern crate rand;

use rand::Rng;
use std::env;
use std::sync::{Mutex, Arc};
use std::thread;

struct Table {
    forks: Vec<Mutex<()>>,
}

fn for_x_secs(max: u32) -> u32 {
    let secs = rand::thread_rng().gen_range(1, max + 1);
    return secs;
}

struct Philosopher {
    name: String,
    born: u32,
    died: u32,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, born: u32, died: u32, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            born: born,
            died: died,
            left: left,
            right: right,
        }
    }

    fn eat(&self, secs: u32, table: &Table) {
        println!("{} blocked on left fork ...", self.name);
        let _left = table.forks[self.left].lock().unwrap();
        println!("{} blocked on right fork ...", self.name);
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is going to eat for {} secs ...", self.name, secs);
        thread::sleep_ms(secs * 1000);
        println!("{} is {} and is done eating!", self.name, self.age());
    }

    fn age(&self) -> String {
        if self.died == 0 {
            return format!("is dead");
        } else {
            return format!("is {} years old", self.died - self.born);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let cmd = args[0].to_string();
    let max_loops: u32;
    let max_secs: u32;
    match args.len() {
        3 => {
            max_loops = args[1].to_string().trim().parse().ok().expect("Failed to parse <max_loops>");
            max_secs = args[2].to_string().trim().parse().ok().expect("Failed to parse <max_secs>");
        }
        _ => {
            println!("Usage: {} <max_loops> <max_secs>", cmd);
            return;
        }
    }

    let table = Arc::new(Table {forks: vec! [
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Noam Chomsky", 1928, 0, 0, 1),
        Philosopher::new("Umberto Eco", 1932, 0, 1, 2),
        Philosopher::new("Iris Murdoch", 1919, 1999, 2, 3),
        Philosopher::new("Alan Turing", 1912, 1954, 3, 4),
        Philosopher::new("Simone Weil", 1909, 1943, 4, 0),
        // one way to avoid a deadlock is make one of the philosophers left-handed
        // Philosopher::new("Simone Weil", 1909, 1943, 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            for x in 0..max_loops {
                println!("{} is going to eat {}/{} ...", p.name, x, max_loops);
                p.eat(for_x_secs(max_secs), &table);
            }
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
