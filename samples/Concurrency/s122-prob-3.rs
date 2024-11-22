// Problem 1: The code is unable to do some other stuff.
// Seems like a message is always available, when you make a call to try_recv().
// Fix the code so that it is able to do other work.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let x = "some_value".to_string();
        println!("Sending value {x}");
        tx.send(x).unwrap();
        thread::sleep(Duration::from_secs(3));
    });

    //t.join(); // Something wrong here
    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is: {received_value}");
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"), // This line never executes.
                                                               // Make approperiate changes in the code, so that this line executes.
        }
    }
}
