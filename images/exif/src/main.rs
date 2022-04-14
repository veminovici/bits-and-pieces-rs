fn main() -> Result<(), exif::Error>{

    for path in &["tests/test.jpg"] {
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        for f in exif.fields() {
            println!("{} {} {}",
                     f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        }
    }

    Ok(())
}
