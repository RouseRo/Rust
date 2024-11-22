// Problem 1: Fix the code below  

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx_clone.send(i).unwrap();
        }
    });

    let tx_clone = tx.clone();
    thread::spawn(move || {
        let my_vec = vec![6, 7, 8, 9, 10];
        for i in my_vec {
            tx_clone.send(i).unwrap();   // fix this line
        }
    });

    for recieved_vals in rx {
        println!("I recieved the value of {}", recieved_vals);
    }
}
