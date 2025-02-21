# Trekname

A Rust library containing Star Trek character names and descriptions from various series including The Original Series, The Next Generation, Deep Space Nine, Voyager, Enterprise, and Discovery.

## Features

- Static compile-time character database using `phf_map`
- Efficient lookups for character descriptions
- Complete collection of major characters from across the Star Trek universe
- Memory-efficient implementation using static strings
- Thread-safe access to character data

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
trekname = "0.1.1"
```

### Example

```rust
use trekname::{get_description, character_exists, get_all_names};

fn main() {
    // Get a character's description
    if let Some(description) = get_description("Kirk") {
        println!("Kirk: {}", description);
    }

    // Check if a character exists
    if character_exists("Picard") {
        println!("Picard exists in the database!");
    }

    // Get all character names
    let names = get_all_names();
    println!("Total characters: {}", names.len());
}
```

## API

### Functions

- `get_description(name: &str) -> Option<&'static str>`: Returns the description for a given character name, if it exists
- `character_exists(name: &str) -> bool`: Returns true if the character exists in the database
- `get_all_names() -> Vec<&'static str>`: Returns a vector of all character names

### Static Data

- `TREK_DESCRIPTIONS`: A static `phf::Map` containing all character names and descriptions

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
