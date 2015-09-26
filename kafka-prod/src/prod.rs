#![allow(unused_must_use)]

extern crate kafka;

use kafka::client::KafkaClient;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let topic: String;
    let message: String;
    match args.len() {
        3 => {
            topic = args[1].to_string();
            message = args[2].to_string();
        }
        _ => {println!("Usage: prod <topic> <message>"); return}
    }

    let mut client = KafkaClient::new(vec!("localhost:9092".to_string()));
    client.load_metadata_all();
    client.send_message(1, 0, topic.clone(), message.clone().into_bytes());
    println!("Sending >{}< to >{}< ...", message, topic);
}
