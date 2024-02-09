pub mod utils;

use std::fs::File;

fn main() {
    // Open the savefile
    let mut file :File = match File::open("pokeemerald.sav") {
        Ok(file) => file,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    // Search for the section offset of the section number 3
    let section_offset = utils::get_section_id(&mut file, 3);
    println!("Section offset: 0x{:X}", section_offset);
}
