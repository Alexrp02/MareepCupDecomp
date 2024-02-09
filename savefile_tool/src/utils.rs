use std::{fs::File, io::{Read, Seek, SeekFrom}};
use byteorder::{ByteOrder, LittleEndian};

use crate::{SECTION_ID_OFFSET, SECTION_ID_SIZE, SECTION_SIZE};


pub fn get_section_id (file :&mut File, section_id: u16) -> u16 {
    let mut offset: u16 = 0x0;

    // Seek to the first section id
    file.seek(SeekFrom::Start(SECTION_ID_OFFSET as u64)).expect("Error seeking to section first id");

    // Read a specific number of bytes
    let mut buffer = vec![0; SECTION_ID_SIZE as usize];
    file.read_exact(&mut buffer).expect("Buffer overflow when reading section id");
 
    // Read bytes and interpret values based on format
    let mut current_section_id = LittleEndian::read_u16(&buffer);

    while current_section_id != section_id {
        offset += SECTION_SIZE;
        // Seek to the next section id
        file.seek(SeekFrom::Start((offset + SECTION_ID_OFFSET) as u64)).expect("Error seeking to next section id");

        // Read a specific number of bytes
        let mut buffer = vec![0; SECTION_ID_SIZE as usize];
        file.read_exact(&mut buffer).expect("Buffer overflow when reading next section id");
    
        // Read bytes and interpret values based on format
        current_section_id = LittleEndian::read_u16(&buffer);
    }

    return offset;
}