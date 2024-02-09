use std::{fs::File, io::{Read, Seek, SeekFrom}};
use byteorder::{ByteOrder, LittleEndian};

// OFFSETS
const SECTION_ID_OFFSET:u16 = 0x0FF4;
const TEAM_SIZE_OFFSET:u16 = 0x0234;

// SIZES
const SECTION_SIZE:u16 = 0x1000;
const SECTION_ID_SIZE:u8 = 0x2;


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

pub fn get_team_size (section_offset :u16, file :&mut File) -> u32 {
    // Seek to the first section id
    file.seek(SeekFrom::Start((section_offset + TEAM_SIZE_OFFSET) as u64)).expect("Error seeking to section first id");

    // Read a specific number of bytes
    let mut buffer = vec![0; 4 as usize];
    file.read_exact(&mut buffer).expect("Buffer overflow when reading section id");
 
    // Read bytes and interpret values based on format
    let team_size = LittleEndian::read_u32(&buffer);

    return team_size;
}