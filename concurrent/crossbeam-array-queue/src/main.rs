use crossbeam::queue::ArrayQueue;
use std::{sync::Arc, thread};

fn run_producer(q: Arc<ArrayQueue<u32>>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - pushing...!", num);
        for _ in 0..20 {
            q.push(num).expect("pushing failed");
        }
    })
}

fn run_consumer(q: Arc<ArrayQueue<u32>>, num: u32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from producer thread {} - popping!", num);
        for _ in 0..20 {
            q.pop();
        }
    })
}

fn main() {
    // ArrayQueue Example
    println!("---------------------------------------");
    println!("Starting ArrayQueue example...");

    // Create a queue with capacity of 100.
    let q: ArrayQueue<u32> = ArrayQueue::new(100);
    let arc_q = Arc::new(q);

    let mut thread_handles_aq: Vec<thread::JoinHandle<()>> = Vec::new();

    // Create the producers.
    for i in 1..5 {
        thread_handles_aq.push(run_producer(arc_q.clone(), i));
    }

    // Create the consumers.
    for i in 1..5 {
        thread_handles_aq.push(run_consumer(arc_q.clone(), i));
    }

    thread_handles_aq
        .into_iter()
        .for_each(|th| th.join().expect("can't join thread"));

    println!("values in q after threads finished: {}", arc_q.len());
    println!("ArrayQueue example finished!");
}
