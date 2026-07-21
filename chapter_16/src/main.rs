use std::{
    cell::RefCell,
    sync::{Arc, Mutex, mpsc},
    thread,
};

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let thread_a = thread::spawn(move || {
        println!("here's a vector {v:?}");
    });

    thread_a.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let thread = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
