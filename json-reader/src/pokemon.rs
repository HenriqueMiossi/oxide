use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct Obj {
    id: i32,
    name: String,
    abilities: Vec<Ability>,
    stats: Vec<Stat>,
    types: Vec<Type>
}

#[derive(Deserialize)]
struct Ability {
    ability: AbilityName
}

#[derive(Deserialize)]
struct AbilityName {
    name: String,
}

#[derive(Deserialize)]
struct Stat {
    base_stat: i32
}

#[derive(Deserialize)]
struct StatName {}

#[derive(Deserialize)]
struct Type {
    r#type: TypeName,
}

#[derive(Deserialize)]
struct TypeName {
    name: String,
}

#[derive(Deserialize)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub abilities: Vec<String>,
    pub stats: Stats,
    pub types: Vec<String>
}

#[derive(Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub special_attack: i32,
    pub special_defense: i32,
    pub speed: i32
}

pub fn parse_json(json_str: &str) -> Pokemon {
    let jobject: Obj = serde_json::from_str(json_str).unwrap();

    get_pokemon_from_json(jobject)
}

fn get_pokemon_from_json(jobject: Obj) -> Pokemon {
    let mut abilities_str: Vec<String> = Vec::new();
    let mut types_str: Vec<String> = Vec::new();
    let mut stats_i32: Vec<i32> = Vec::new();

    for i in jobject.abilities.iter() {
        abilities_str.push(i.ability.name.clone());
    }

    for i in jobject.types.iter() {
        types_str.push(i.r#type.name.clone());
    }

    for i in jobject.stats.iter() {
        stats_i32.push(i.base_stat);
    }

    let pokemon_stats = Stats {
        hp: stats_i32[0],
        attack: stats_i32[1],
        defense: stats_i32[2],
        special_attack: stats_i32[3],
        special_defense: stats_i32[4],
        speed: stats_i32[5]
    };

    let pokemon = Pokemon {
        id: jobject.id,
        name: jobject.name,
        abilities: abilities_str,
        types: types_str,
        stats: pokemon_stats
    };

    pokemon
}
