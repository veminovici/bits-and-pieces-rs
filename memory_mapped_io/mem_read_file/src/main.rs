use memmap2::Mmap;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    // Open an existing file
    let file = File::open("example.dat")?;

    // Memory-map the file for reading
    let mmap = unsafe { Mmap::map(&file)? };

    // Read some data from the memory-mapped file
    // Here, we'll just print out the first 10 bytes
    let data = &mmap[0..10];

    println!("First 10 bytes: {:?}", data);

    Ok(())
}
