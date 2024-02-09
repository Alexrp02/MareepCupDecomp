use std::{fs::File, io::{Read, Seek, SeekFrom}, vec};

use byteorder::{ByteOrder, LittleEndian};

// OFFSETS;
const OT_ID_OFFSET:u16 = 0x04;
const DATA_OFFSET:u16 = 0x20;

// SIZES
const DATA_SIZE:u8 = 48;

pub struct Pokemon {
    personality: u32,
    ot_id: u32,
    level :u8,
    data: Vec<u8>,
    decryption_key: u32,
}

pub trait Getters {
    fn get_personality(&self) -> u32;
    fn get_ot_id(&self) -> u32;
    fn get_level(&self) -> u8 ;
    fn get_data(&self) -> &Vec<u8> ;
    fn get_decryption_key(&self) -> u32;
}

trait Readers {
    fn read_personality(pokemon_offset :u16, file :&mut File) -> u32;
    fn read_ot_id(pokemon_offset :u16, file :&mut File) -> u32;
}

trait Decryption{
    fn calculate_decryption_key(personality:u32, ot_id:u32) -> u32;
    fn get_encrypted_data(file: &mut File, pokemon_offset:u16) -> [u8; DATA_SIZE as usize];
    fn decrypt_data(data: &[u8], decryption_key:u32) -> Vec<u8>;
}

impl Pokemon {
    pub fn new(pokemon_offset :u16, file :&mut File) -> Self {
        let personality = Self::read_personality(pokemon_offset, file);
        let ot_id = Self::read_ot_id(pokemon_offset, file);
        let decryption_key = Self::calculate_decryption_key(personality, ot_id);
        let encrypted_data : [u8; DATA_SIZE as usize] = Self::get_encrypted_data(file, pokemon_offset);
        let data = Self::decrypt_data(&encrypted_data, decryption_key);
        Pokemon {
            personality,
            ot_id,
            level: 50,
            data,
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
    fn get_data(&self) -> &Vec<u8> {
        &self.data
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

    fn get_encrypted_data(file: &mut File, pokemon_offset:u16) -> [u8; DATA_SIZE as usize] {
        file.seek(SeekFrom::Start((pokemon_offset + DATA_OFFSET) as u64)).expect("Error seeking to pokemon data");
    
        let mut encrypted_data = [0; DATA_SIZE as usize];
        file.read_exact(&mut encrypted_data).expect("Buffer overflow when reading pokemon encrypted data");
    
        return encrypted_data;
    }

    fn decrypt_data(data: &[u8], decryption_key:u32) -> Vec<u8> {
        let mut decrypted_data = Vec::with_capacity(data.len()); // Pre-allocate for efficiency

        // Handle cases where data length is not a multiple of 4 bytes
        if data.len() % 4 != 0 {
            panic!("Encrypted data length must be a multiple of 4 bytes for 32-bit XOR decryption.");
        }

        for chunk in data.chunks_exact(4) {
            // Convert 4-byte chunk to u32
            let encrypted_u32 = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

            // Decrypt using XOR with secret key
            let decrypted_u32 = encrypted_u32 ^ decryption_key;

            // Convert back to bytes and push to decrypted data
            let decrypted_bytes = decrypted_u32.to_le_bytes();
            decrypted_data.extend_from_slice(&decrypted_bytes);
        }

        decrypted_data
    }

}