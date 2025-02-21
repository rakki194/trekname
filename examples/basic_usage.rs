use trekname::{get_description, character_exists, get_all_names};

fn main() {
    // Example 1: Get a specific character's description
    if let Some(description) = get_description("Kirk") {
        println!("Kirk: {}", description);
    }

    // Example 2: Check if characters exist
    let characters = ["Picard", "NonExistentCharacter", "Spock"];
    for character in characters {
        if character_exists(character) {
            println!("{} exists in the database!", character);
        } else {
            println!("{} does not exist in the database.", character);
        }
    }

    // Example 3: List all characters
    let names = get_all_names();
    println!("\nTotal number of characters: {}", names.len());
    println!("\nFirst 5 characters in the database:");
    for name in names.iter().take(5) {
        if let Some(description) = get_description(name) {
            println!("{}: {}", name, description);
        }
    }
} 