use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "./../example_mem_inter_proc.dat";

    let file = OpenOptions::new().read(true).write(true).open(file_path)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // Modify a portion of the mapped memory
    let new_data = b"Rust";

    for (i, byte) in new_data.iter().enumerate() {
        mmap[i] = *byte;
    }

    mmap.flush()?;

    println!("Memory-mapped file updated.");

    Ok(())
}
