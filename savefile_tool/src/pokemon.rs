use std::{fs::File, io::{Read, Seek, SeekFrom}};

use byteorder::{ByteOrder, LittleEndian};

// OFFSETS;
const OT_ID_OFFSET:u16 = 0x04;

pub struct Pokemon {
    personality: u32,
    ot_id: u32,
    level :u8,
    data: [u8; 48],
    decryption_key: u32,
}

pub trait Getters {
    fn get_personality(&self) -> u32;
    fn get_ot_id(&self) -> u32;
    fn get_level(&self) -> u8 ;
    fn get_data(&self) -> [u8; 48] ;
    fn get_decryption_key(&self) -> u32;
}

trait Readers {
    fn read_personality(pokemon_offset :u16, file :&mut File) -> u32;
    fn read_ot_id(pokemon_offset :u16, file :&mut File) -> u32;
}

trait Decryption{
    fn calculate_decryption_key(personality:u32, ot_id:u32) -> u32;
}

impl Pokemon {
    pub fn new(pokemon_offset :u16, file :&mut File) -> Self {
        let personality = Self::read_personality(pokemon_offset, file);
        let ot_id = Self::read_ot_id(pokemon_offset, file);
        let decryption_key = Self::calculate_decryption_key(personality, ot_id);
        Pokemon {
            personality,
            ot_id,
            level: 50,
            data: [0; 48],
            decryption_key,
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

    fn get_decryption_key(&self) -> u32 {
        self.decryption_key
    }
}

impl Readers for Pokemon {
    fn read_personality(pokemon_offset :u16, file :&mut File) -> u32 {
        file.seek(SeekFrom::Start(pokemon_offset as u64)).expect("Error seeking to pokemon offset");
    
        let mut buffer = vec![0; 4];
        file.read_exact(&mut buffer).expect("Buffer overflow when reading pokemon personality");
    
        let personality = LittleEndian::read_u32(&buffer);
        return personality;
    }

    fn read_ot_id(pokemon_offset :u16, file :&mut File) -> u32 {
        file.seek(SeekFrom::Start((pokemon_offset + OT_ID_OFFSET) as u64)).expect("Error seeking to pokemon offset");
    
        let mut buffer = vec![0; 4];
        file.read_exact(&mut buffer).expect("Buffer overflow when reading pokemon ot id");
    
        let ot_id = LittleEndian::read_u32(&buffer);
        return ot_id;
    }
}

impl Decryption for Pokemon {
    fn calculate_decryption_key(personality:u32, ot_id:u32) -> u32 {
        let key = personality ^ ot_id;
        return key;
    }
}