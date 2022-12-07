use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // send_once();
    // send_multiple();
    multiple_producers();
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("there"),
            String::from("from"),
            String::from("thread 1"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("this"),
            String::from("is"),
            String::from("now"),
            String::from("thread 2"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got message: {}", received);
    }
}

fn send_multiple() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("there"),
            String::from("from"),
            String::from("thread 1"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got message: {}", received);
    }
}

fn send_once() {
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
