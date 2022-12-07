use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    mutex_101();
    mutex_threads();
}

fn mutex_threads() {
    println!("==================== Using atomics across threads ====================");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("Incrementing counter from {} to {}", *num, *num + 1);
            thread::sleep(Duration::from_secs(1));
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter result: {}", *counter.lock().unwrap());
}

fn mutex_101() {
    println!("==================== MUTEX 101 ====================");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        // first, MutexGuard implements the Deref trait,
        // so we can use the `*` operator directly
        *num = 10;
        // MutexGuard implements the Drop trait
        // which releases the lock on the Mutex automatically
        // when it goes out of scope.
    }

    // lock was dropped, now can we access `m`.
    println!("m = {:?}", m);
}
