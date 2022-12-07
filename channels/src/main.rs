use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi there!");
        match tx.send(val) {
            Ok(_) => println!("Message sent!"),
            Err(e) => println!("Things went wrong {}", e),
        };
    });

    match rx.recv() {
        Ok(msg) => println!("Got message: {}", msg),
        Err(e) => println!("Error receiving msg {}", e),
    };
}
