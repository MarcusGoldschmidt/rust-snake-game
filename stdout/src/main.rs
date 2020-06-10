use std::io::{self};
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a simple streaming channel
    let (tx, rx) = channel();

    thread::spawn(move || {
        let stdin = io::stdin();

        loop {
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
        }
    });

    let mut seconds = 0;

    loop {
        match rx.try_recv() {
            Ok(d) => println!("Receive {}", d),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        thread::sleep(Duration::from_secs(1));
        seconds += 1;
        println!(">Time: {}", seconds);
    }
}
