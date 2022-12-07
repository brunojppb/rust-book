use std::{thread, time::Duration};

fn main() {
    handle_and_join();
    move_thread_ref()
}

fn move_thread_ref() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vector values are: {:?}", v);
    });

    handle.join().unwrap();
}

fn handle_and_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("num {} from spanmed thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("num {} from MAIN thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
