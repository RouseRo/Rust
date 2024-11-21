// Problem 2: Complete the code below

use std::sync::mpsc;
use std::thread;

fn thread_fn(d: i32, tx: mpsc::Sender<i32>) {
    thread::spawn(move || {
        println!("{} send!", d);
        tx.send(d).unwrap();
        // Add code for sending d 
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        // Add code for calling the function with value i and a clone of tx
        thread_fn(i, tx.clone());
    }

    drop(tx);

    for recieving_val in rx {
        println!("{} received!", recieving_val);
    }
}
