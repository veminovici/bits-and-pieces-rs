use crossbeam::channel::{unbounded, Receiver, Sender};
use std::thread;

fn run_producer_chan(s: Sender<u32>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - pushing...!", num);
        for _ in 0..1000 {
            s.send(num).expect("send failed");
        }
    })
}

fn run_consumer_chan(r: Receiver<u32>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut i = 0;
        println!("Hello from producer thread {} - popping!", num);
        loop {
            if let Err(_) = r.recv() {
                println!(
                    "last sender dropped - stopping consumer thread, messages received: {}",
                    i
                );
                break;
            }
            i += 1;
        }
    })
}

fn main() {
    // channel Example
    println!("---------------------------------------");
    println!("Starting channel example...");

    // Create the channel.
    let (s, r) = unbounded();

    // Create the producers.
    for i in 1..5 {
        run_producer_chan(s.clone(), i);
    }
    drop(s);

    // Create the consumers.
    for i in 1..5 {
        run_consumer_chan(r.clone(), i);
    }

    println!("channel example finished!");
}
