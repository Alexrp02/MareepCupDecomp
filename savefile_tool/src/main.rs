pub mod utils;
pub mod pokemon;

use std::{fs::File, io::Write};
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

    // Get the team size
    let team_size = utils::get_team_size(section_offset, &mut file);
    println!("Team size: {}", team_size);

    let mut team: String = "".to_string();
    for i in 0..team_size {
        // Get the pokemon offset
        let pokemon_offset = utils::get_pokemon_offset(section_offset, i as u16);
        println!("Pokemon {} offset: 0x{:X}", i, pokemon_offset);

        // Create a new Pokemon object
        let mut pokemon = pokemon::Pokemon::new(pokemon_offset, &mut file);

        pokemon.read_all_data();

        // Print showdown
        println!("Pokemon {} as showdown:\n{}", i, pokemon.get_as_showdown());
        team.push_str(&pokemon.get_as_showdown());
    }

    // Print team
    println!("Team as showdown:\n{}", team);

    // Save team to file
    let mut team_file = match File::create("team.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("Error while creating the team file: {}", error);
            return;
        }
    };
    match team_file.write_all(team.as_bytes()) {
        Ok(_) => println!("Team saved to file"),
        Err(error) => println!("Error while writing the team file: {}", error)
    }
}
