pub mod utils;
pub mod pokemon;

use std::fs::File;
use crate::pokemon::{Getters, Readers};

const POKEMON_DATA_INDEX :u16 = 1;

fn main() {
    // Open the savefile
    let mut file :File = match File::open("pokeemerald.sav") {
        Ok(file) => file,
        Err(error) => {
            println!("Error while opening the savefile: {}", error);
            return;
        }
    };

    // Search for the section offset of the section number 1 (Pokemon team data)
    let section_offset = utils::get_section_id(&mut file, POKEMON_DATA_INDEX);
    println!("Pokemon section offset: 0x{:X}", section_offset);

    // Get the first pokemon offset
    let pokemon_offset = utils::get_pokemon_offset(section_offset, 0);
    println!("First pokemon offset: 0x{:X}", pokemon_offset);

    // Get the team size
    let team_size = utils::get_team_size(section_offset, &mut file);
    println!("Team size: {}", team_size);

    // Create a new Pokemon object
    let mut pokemon = pokemon::Pokemon::new(pokemon_offset, &mut file);
    println!("The personality mod 24 is {}", pokemon.get_personality() % 24);

    // Print the ot id
    println!("First pokemon ot id: {}", pokemon.get_ot_id());

    // Print the decryption key
    println!("First pokemon decryption key: 0x{:X}", pokemon.get_decryption_key());

    pokemon.read_all_data();

    // Print the specie
    println!("First pokemon specie: {}", pokemon.get_specie());

}
