mod pokemon;

use pokemon::{parse_json, Pokemon};
use std::fs;
use std::io;
use std::io::Write;

fn read_file(path: &str) -> String {
    let data: String = fs::read_to_string(path).expect("Unable to read file");

    return data;
}

fn main() {
    let data: String = read_file("C:\\Users\\henri\\Desktop\\Projects\\Rust\\oxide\\json-reader\\src\\PokeAPI - Ditto Sample.json");

    let pokemon: Pokemon = parse_json(&data);

    println!("The pokémon {} have this id: {}", pokemon.name, pokemon.id);
    println!("It's abilities are: {:?}", pokemon.abilities);
    print!("It's stats are:");
    io::stdout().flush().unwrap();

    let s = &pokemon.stats;

    println!(
        "HP: {}, Attack: {}, Defense: {}, Special Attack: {}, Special Defense: {}, Speed: {}",
        s.hp, s.attack, s.defense, s.special_attack, s.special_defense, s.speed
    );
}
