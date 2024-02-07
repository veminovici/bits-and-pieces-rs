use memmap2::MmapMut;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;

struct SimpleDB {
    mmap: MmapMut,
    index: HashMap<String, (usize, usize)>, // Key to (offset, length)
}

impl SimpleDB {
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
            index: HashMap::new(),
        })
    }

    fn insert(&mut self, key: &str, value: &[u8]) -> io::Result<()> {
        let offset = self.mmap.len(); // Append to the end
        let length = value.len();
        // if offset + length > self.mmap.() {
        //     return Err(io::Error::new(
        //         io::ErrorKind::OutOfMemory,
        //         "Database is full",
        //     ));
        // }
        self.mmap[offset..offset + length].copy_from_slice(value);
        self.index.insert(key.to_string(), (offset, length));
        Ok(())
    }

    fn get(&self, key: &str) -> Option<&[u8]> {
        self.index
            .get(key)
            .map(|&(offset, length)| &self.mmap[offset..offset + length])
    }
}

fn main() -> io::Result<()> {
    let mut db = SimpleDB::new("simple_db.dat", 1024 * 1024)?; // 1 MB database

    db.insert("hello", b"world")?;

    db.insert("foo", b"bar")?;

    if let Some(value) = db.get("hello") {
        println!("Value for 'hello': {:?}", value);
    }

    Ok(())
}
