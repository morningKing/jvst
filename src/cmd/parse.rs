use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::path::Path;

pub fn show_clz_bytes(clz_abs_path: &str) -> std::io::Result<()> {
    // let clz_path = Path::new(clz_abs_path);
    println!("debug {:}", clz_abs_path);
    let f = File::open(clz_abs_path)?;
    let mut reader = BufReader::new(f);

    // let content: String;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    println!("{:}", buffer);
    // let mut line = String::new();
    // let len = reader.read_line(&mut line)?;
    // println!("First line is {} bytes long", len);
    Ok(())
}
