use std::error::Error;
use std::fs::File;
use std::str;
use std::io::prelude::*;

pub fn read_event() -> Result<(), Box<dyn Error>> {
    let mut contents = File::open("/dev/input/event6")?;
    let mut buf: [u8; 24] = [0; 24];
    println!("NEPEEEEE");

    loop {
        let size = match contents.read(&mut buf) {
            Ok(n) => n,
            Err(_) => panic!("Error reading file"),
        };

        println!("{}:{}", size, str::from_utf8(&buf)?);
    }

    Ok(())
}
