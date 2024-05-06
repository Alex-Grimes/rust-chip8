use std::fs::File;
use std::io::prelude::*;

pub struct CartridgeModule {
    pub rom: [u8; 3584],
    pub size: usize,
}

impl CartridgeModule {
    pub fn new(filename: &str) -> Self {
        let mut f = Fil::open(filename).expect("file not found");
        let mut buffer = [0u8; 3584];

        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };

        CartridgeModule {
            rom: buffer,
            size: bytes_read,
        }
    }
}
