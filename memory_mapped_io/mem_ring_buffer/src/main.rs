use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io;

struct RingBuffer {
    mmap: MmapMut,
    capacity: usize,
    head: usize,
    tail: usize,
}

impl RingBuffer {
    fn new(file_path: &str, size: usize) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        file.set_len(size as u64)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(Self {
            mmap,
            capacity: size,
            head: 0,
            tail: 0,
        })
    }

    fn write(&mut self, data: &[u8]) {
        for &byte in data {
            self.mmap[self.head] = byte;
            self.head = (self.head + 1) % self.capacity;
            if self.head == self.tail {
                self.tail = (self.tail + 1) % self.capacity; // Overwrite oldest data
            }
        }
    }
    // Additional methods for reading, seeking, etc., can be added here
}

fn main() -> io::Result<()> {
    let mut ring_buffer = RingBuffer::new("example.dat", 1024)?;
    ring_buffer.write(b"Hello, Ring Buffer!");
    Ok(())
}
