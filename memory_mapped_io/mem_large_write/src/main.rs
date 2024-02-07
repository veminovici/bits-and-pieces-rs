use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io;

fn manipulate_large_file(file_path: &str) -> io::Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // Example manipulation: zero out every other byte in a large file
    for i in (0..mmap.len()).step_by(2) {
        mmap[i] = 0;
    }

    mmap.flush()?; // Ensure changes are written back to the file

    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = "example.dat";
    manipulate_large_file(file_path)
}
