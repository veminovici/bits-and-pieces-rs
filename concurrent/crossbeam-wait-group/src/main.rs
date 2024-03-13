use crossbeam::sync::WaitGroup;
use rand::Rng;
use std::thread;

fn do_work(thread_num: i32) {
    let num = rand::thread_rng().gen_range(100..500);
    thread::sleep(std::time::Duration::from_millis(num));
    let mut sum = 0;
    for i in 0..10 {
        sum += sum + num * i;
    }
    println!(
        "thread {} calculated sum: {}, num: {}",
        thread_num, sum, num
    );
    thread::sleep(std::time::Duration::from_millis(num));
}

fn main() {
    // WaitGroup Example
    println!("---------------------------------------");
    println!("Starting WaitGroup example...");

    // Create a waiting group
    let wg = WaitGroup::new();

    for i in 0..50 {
        let wg_clone = wg.clone();
        thread::spawn(move || {
            do_work(i);
            drop(wg_clone);
        });
    }

    println!("waiting for all threads to finish...!");
    wg.wait();
    println!("all threads finished!");
    println!("WaitGroup example finished!");
}
