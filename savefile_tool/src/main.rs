pub mod constants;
pub mod utils;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{ByteOrder, LittleEndian};

use crate::constants::*;

fn main() {
    // Open the savefile
    let mut file :File = match File::open("pokeemerald.sav") {
        Ok(file) => file,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    // Seek to the section id
    match file.seek(SeekFrom::Start(SECTION_ID_OFFSET as u64)) {
        Ok(_) => {},
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
        
    };

    // Read a specific number of bytes
    let num_bytes = SECTION_ID_SIZE; // Replace with desired number of bytes to read
    let mut buffer = vec![0; num_bytes as usize];
    file.read_exact(&mut buffer).unwrap();

    // Read bytes and interpret values based on format
    let value = LittleEndian::read_u16(&buffer); // Example for reading a u32
    println!("Read value from offset 0x{:x}: {}", SECTION_ID_OFFSET, value);
}
