use std::{fs::File, io::{Read, Seek, SeekFrom}};

use byteorder::{ByteOrder, LittleEndian};

// OFFSETS
const TEAM_POKEMON_OFFSET:u16 = 0x0238;

pub struct Pokemon {
    personality: u32,
    ot_id: u32,
    level :u8,
    data: [u8; 48],
}

pub trait Getters {
    fn get_personality(&self) -> u32;
    fn get_ot_id(&self) -> u32;
    fn get_level(&self) -> u8 ;
    fn get_data(&self) -> [u8; 48] ;
}

impl Pokemon {
    pub fn new(pokemon_offset :u16, file :&mut File) -> Self {
        read_personality(pokemon_offset, file);
        Pokemon {
            personality: 0,
            ot_id: 0,
            level: 50,
            data: [0; 48],
        }
    }
}

impl Getters for Pokemon {
    fn get_personality(&self) -> u32 {
        self.personality
    }
    fn get_ot_id(&self) -> u32 {
        self.ot_id
    }
    fn get_level(&self) -> u8 {
        self.level
    }
    fn get_data(&self) -> [u8; 48] {
        self.data
    }
}

fn read_personality(pokemon_offset :u16, file :&mut File) -> u32 {
    // Seek to the first section id
    file.seek(SeekFrom::Start((pokemon_offset + TEAM_POKEMON_OFFSET) as u64)).expect("Error seeking to pokemon offset");

    // Read a specific number of bytes
    let mut buffer = vec![0; 4];
    file.read_exact(&mut buffer).expect("Buffer overflow when reading pokemon personality");

    // Read bytes and interpret values based on format
    let personality = LittleEndian::read_u32(&buffer);
    return personality;
}