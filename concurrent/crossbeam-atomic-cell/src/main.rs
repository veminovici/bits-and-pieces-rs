use crossbeam::atomic::AtomicCell;
use std::{sync::Arc, thread};

fn run_thread(val: Arc<AtomicCell<u32>>, num: u32, store: bool) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if store {
            val.fetch_add(1);
        }
        println!("Hello from thread {}! value: {}", num, val.load());
    })
}
fn main() {
    // AtomicCell Example
    println!("Starting AtomicCell example...");

    // The cell to be updated.
    let atomic_value: AtomicCell<u32> = AtomicCell::new(12);
    let arc = Arc::new(atomic_value);

    // Create the threads - pnly half of them will write into the value.
    let mut thread_handles_ac: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..10 {
        thread_handles_ac.push(run_thread(arc.clone(), i, i % 2 == 0));
    }

    thread_handles_ac
        .into_iter()
        .for_each(|th| th.join().expect("can't join thread"));

    println!("value after threads finished: {}", arc.load());
    println!("AtomicCell example finished!");
}
