use std::{
    ascii::AsciiExt, fs::File, io::{BufRead, BufReader, Read, Seek, SeekFrom}, vec
};

use byteorder::{ByteOrder, LittleEndian};

// OFFSETS;
const OT_ID_OFFSET: u16 = 0x04;
const DATA_OFFSET: u16 = 0x20;
const SPECIE_OFFSET: u16 = 0x0;
const ITEM_HELD_OFFSET: u16 = 0x02;

// SIZES
const DATA_SIZE: u8 = 48;
const SUBDATA_SIZE: u16 = 12;

const NATURE_NAMES_FILE: &str = "text/nature_names.txt" ;
const POKEMON_NAMES_FILE: &str = "text/species_names.txt" ;

const FORMATS: [&str; 24] = [
    "GAEM", "GAME", "GEAM", "GEMA", "GMAE", "GMEA", "AGEM", "AGME", "AEGM", "AEMG", "AMGE", "AMEG",
    "EGAM", "EGMA", "EAGM", "EAMG", "EMGA", "EMAG", "MGAE", "MGEA", "MAGE", "MAEG", "MEGA", "MEAG",
];

pub struct Pokemon {
    personality: u32,
    specie: u16,
    item_held: u16,
    moves: Vec<u16>,
    evs: Vec<u8>,
    ivs: Vec<u8>,
    nature: u8,
    second_ability: bool,
    ot_id: u32,
    level: u8,
    data: Vec<u8>,
    decryption_key: u32,
    format: u16,
}

pub trait Getters {
    fn get_personality(&self) -> u32;
    fn get_specie(&self) -> u16;
    fn get_ot_id(&self) -> u32;
    fn get_level(&self) -> u8;
    fn get_data(&self) -> &Vec<u8>;
    fn get_decryption_key(&self) -> u32;
    fn get_as_showdown(&self) -> String;
    fn get_correct_name(&self, pokemon_names: Vec<String>) -> String;
}

pub trait Readers {
    fn read_personality(pokemon_offset: u16, file: &mut File) -> u32;
    fn read_ot_id(pokemon_offset: u16, file: &mut File) -> u32;
    fn read_species(data: &Vec<u8>) -> u16;
    fn calculate_format(personality: u32) -> u16;
    fn read_all_data(&mut self);
}

trait Decryption {
    fn calculate_decryption_key(personality: u32, ot_id: u32) -> u32;
    fn get_encrypted_data(file: &mut File, pokemon_offset: u16) -> [u8; DATA_SIZE as usize];
    fn decrypt_data(data: &[u8], decryption_key: u32) -> Vec<u8>;
}

pub trait Names {
    fn get_natures(filename: &str) -> Vec<String>;
}

impl Pokemon {
    pub fn new(pokemon_offset: u16, file: &mut File) -> Self {
        let personality = Self::read_personality(pokemon_offset, file);
        let ot_id = Self::read_ot_id(pokemon_offset, file);
        let decryption_key = Self::calculate_decryption_key(personality, ot_id);
        let encrypted_data: [u8; DATA_SIZE as usize] =
            Self::get_encrypted_data(file, pokemon_offset);
        let data = Self::decrypt_data(&encrypted_data, decryption_key);
        let format = Self::calculate_format(personality);
        println!("{:?}", get_natures("text/nature_names.txt"));
        Pokemon {
            personality,
            specie: 0,
            item_held: 0,
            moves: vec![],
            evs: vec![0, 0, 0, 0, 0, 0],
            ivs: vec![0, 0, 0, 0, 0, 0],
            nature: (personality % 25) as u8,
            second_ability: (personality >> 31) == 1,
            ot_id,
            level: 50,
            data,
            decryption_key,
            format,
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

    fn get_specie(&self) -> u16 {
        self.specie
    }

    fn get_correct_name(&self, pokemon_names: Vec<String>) -> String {
        let mut correct_name = "".to_string();
        let name = pokemon_names[self.specie as usize].clone();
        let name_without_spaces = name.split("_") ;
        for word in name_without_spaces {
            let correct_word = word.chars().enumerate().map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() }).collect::<String>(); 
            correct_name.push_str(&correct_word) ;
            correct_name.push_str(" ");
        }
        correct_name
    }

    fn get_as_showdown(&self) -> String {
        let nature_names = get_natures(NATURE_NAMES_FILE) ;
        let pokemon_names = get_pokemon_names(POKEMON_NAMES_FILE) ;
        let correct_name = self.get_correct_name(pokemon_names);

        let mut showdown = String::new();
        showdown.push_str(&format!("{} @ {}", correct_name, self.item_held));
        showdown.push_str("\n");
        showdown.push_str("Level: 50");
        showdown.push_str("\n");
        showdown.push_str(&format!("Ability: {}", self.second_ability));
        showdown.push_str("\n");
        showdown.push_str(&format!("{} Nature", nature_names[self.nature as usize]));
        showdown.push_str("\n");
        showdown.push_str(&format!(
            "IVs: {}, {}, {}, {}, {}, {}",
            self.ivs[0], self.ivs[1], self.ivs[2], self.ivs[3], self.ivs[4], self.ivs[5]
        ));
        showdown.push_str("\n");
        for i in 0..4 {
            showdown.push_str(&format!("- {}\n", self.moves[i]));
        }
        showdown
    }
}

impl Readers for Pokemon {
    fn read_personality(pokemon_offset: u16, file: &mut File) -> u32 {
        file.seek(SeekFrom::Start(pokemon_offset as u64))
            .expect("Error seeking to pokemon offset");

        let mut buffer = vec![0; 4];
        file.read_exact(&mut buffer)
            .expect("Buffer overflow when reading pokemon personality");

        let personality = LittleEndian::read_u32(&buffer);
        return personality;
    }

    fn read_ot_id(pokemon_offset: u16, file: &mut File) -> u32 {
        file.seek(SeekFrom::Start((pokemon_offset + OT_ID_OFFSET) as u64))
            .expect("Error seeking to pokemon offset");

        let mut buffer = vec![0; 4];
        file.read_exact(&mut buffer)
            .expect("Buffer overflow when reading pokemon ot id");

        let ot_id = LittleEndian::read_u32(&buffer);
        return ot_id;
    }

    fn read_species(data: &Vec<u8>) -> u16 {
        let species =
            LittleEndian::read_u16(&data[(SPECIE_OFFSET) as usize..(SPECIE_OFFSET) as usize + 2]);
        println!("Species: {}", species);
        return species;
    }

    fn calculate_format(personality: u32) -> u16 {
        let format = personality % 24;
        return format as u16;
    }

    fn read_all_data(&mut self) {
        let structure = FORMATS[self.format as usize];
        for n in 0..4 {
            match structure.chars().nth(n).expect("Error getting the char") {
                'G' => {
                    println!("G");
                    self.specie = LittleEndian::read_u16(
                        &self.data[(SPECIE_OFFSET + SUBDATA_SIZE * (n as u16)) as usize
                            ..(SPECIE_OFFSET + SUBDATA_SIZE * (n as u16)) as usize + 2],
                    );
                    self.item_held = LittleEndian::read_u16(
                        &self.data[(ITEM_HELD_OFFSET + SUBDATA_SIZE * (n as u16)) as usize
                            ..(ITEM_HELD_OFFSET + SUBDATA_SIZE * (n as u16)) as usize + 2],
                    );
                }
                'A' => {
                    println!("A");
                    // Read the four moves
                    for i in 0..4 {
                        let move_id = LittleEndian::read_u16(
                            &self.data[(SUBDATA_SIZE * (n as u16) + 2 * i) as usize
                                ..(SUBDATA_SIZE * (n as u16) + 2 * i) as usize + 2],
                        );
                        self.moves.push(move_id);
                    }
                    println!("{:?}", self.moves);
                }
                'E' => {
                    println!("E");
                    for i in 0..6 {
                        let ev = self.data[(SUBDATA_SIZE * (n as u16) + i) as usize];
                        self.evs[i as usize] = ev;
                    }
                    println!("EV's: {:?}", self.evs);
                }
                'M' => {
                    println!("M");
                    let data_bytes = &self.data[(SUBDATA_SIZE * (n as u16)) as usize
                        ..(SUBDATA_SIZE * (n as u16)) as usize + SUBDATA_SIZE as usize];
                    let data_int = LittleEndian::read_u32(data_bytes);
                    // Mask lower 30 bits
                    let masked_data = data_int & ((1 << 30) - 1);
                    for i in 0..6 {
                        let iv = (masked_data >> (5 * i)) & 0b11111;
                        self.ivs[i as usize] = iv as u8;
                    }
                    println!("IV's: {:?}", self.ivs);
                }
                _ => {
                    println!("Error");
                }
            }
        }
    }
}

impl Decryption for Pokemon {
    fn calculate_decryption_key(personality: u32, ot_id: u32) -> u32 {
        let key = personality ^ ot_id;
        return key;
    }

    fn get_encrypted_data(file: &mut File, pokemon_offset: u16) -> [u8; DATA_SIZE as usize] {
        file.seek(SeekFrom::Start((pokemon_offset + DATA_OFFSET) as u64))
            .expect("Error seeking to pokemon data");

        let mut encrypted_data = [0; DATA_SIZE as usize];
        file.read_exact(&mut encrypted_data)
            .expect("Buffer overflow when reading pokemon encrypted data");

        return encrypted_data;
    }

    fn decrypt_data(data: &[u8], decryption_key: u32) -> Vec<u8> {
        let mut decrypted_data = Vec::with_capacity(data.len()); // Pre-allocate for efficiency

        // Handle cases where data length is not a multiple of 4 bytes
        if data.len() % 4 != 0 {
            panic!(
                "Encrypted data length must be a multiple of 4 bytes for 32-bit XOR decryption."
            );
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

fn get_natures(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Error opening the nature names file");
    let reader = BufReader::new(file);
    let mut nature_names: Vec<String> = vec![];
    for line in reader.lines() {
        nature_names.push(line.expect("Error getting a line in the nature names file"));
    }
    return nature_names;
}

fn get_pokemon_names(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Error opening the pokemon names file");
    let reader = BufReader::new(file);
    let mut pokemon_names: Vec<String> = vec![];
    for line in reader.lines() {
        pokemon_names.push(line.expect("Error getting a line in the pokemon names file"));
    }
    return pokemon_names;
}