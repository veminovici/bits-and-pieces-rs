use memmap2::Mmap;
use std::fs::File;
use std::io;
use std::sync::Arc;
use std::thread;

fn main() -> io::Result<()> {
    let file = File::open("example.dat")?;

    let mmap = unsafe { Mmap::map(&file)? };

    let mmap_arc = Arc::new(mmap);

    let mut handles = vec![];

    for _ in 0..4 {
        // Create 4 threads
        let mmap_clone = Arc::clone(&mmap_arc);
        let handle = thread::spawn(move || {
            // Each thread reads from the memory-mapped file
            let data = &mmap_clone[0..10]; // Example: Read first 10 bytes
            println!("Thread read: {:?}", data);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
