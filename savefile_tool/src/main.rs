pub mod utils;
pub mod pokemon;

use std::fs::File;
use crate::pokemon::Getters;

fn main() {
    // Open the savefile
    let mut file :File = match File::open("pokeemerald.sav") {
        Ok(file) => file,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    // Search for the section offset of the section number 1 (Pokemon team data)
    let section_offset = utils::get_section_id(&mut file, 1);
    println!("Pokemon section offset: 0x{:X}", section_offset);

    // Get the team size
    let team_size = utils::get_team_size(section_offset, &mut file);
    println!("Team size: {}", team_size);

    // Create a new Pokemon object
    let pokemon = pokemon::Pokemon::new(section_offset, &mut file);
    println!("First pokemon personality: 0x{:X}", pokemon.get_personality());
}
