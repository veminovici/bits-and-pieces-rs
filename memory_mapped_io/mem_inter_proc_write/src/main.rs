use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let file_path = "./../example_mem_inter_proc.dat";

    let message = b"IPC using mmap in Rust!";

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    file.set_len(message.len() as u64)?;

    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    mmap[..message.len()].copy_from_slice(message);

    mmap.flush()?;

    println!("Message written to shared memory.");

    Ok(())
}
