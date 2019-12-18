use std::fs::File;
use std::io::prelude::*;

pub fn show_clz_bytes(clz_abs_path: &str) -> std::io::Result<()> {
    let mut f = File::open(clz_abs_path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    for buf in buffer {
        print!("{}", buf);
    }
    Ok(())
}
