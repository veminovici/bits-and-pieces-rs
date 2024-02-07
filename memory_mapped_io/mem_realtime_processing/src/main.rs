use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io;
use std::time::{Duration, Instant};

fn process_real_time_data(file_path: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    file.set_len(1024 * 1024)?; // 1 MB

    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(10) {
        // Process for 10 seconds
        let timestamp = start.elapsed().as_micros() as u32;
        let data = timestamp.to_ne_bytes(); // Example data: current timestamp

        // Write data to a specific location, e.g., beginning of the mmap
        mmap[0..data.len()].copy_from_slice(&data);

        // Simulate real-time data processing by sleeping for a short duration
        std::thread::sleep(Duration::from_micros(1));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    process_real_time_data("example.dat")
}
