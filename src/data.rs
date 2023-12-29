use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

const AREA_JSON_STR: &str = include_str!("data/area.json");
const FAMILY_JSON_STR: &str = include_str!("data/family.json");
const MONSTER_JSON_STR: &str = include_str!("data/monster.json");
const RANK_JSON_STR: &str = include_str!("data/rank.json");
const SEASON_JSON_STR: &str = include_str!("data/season.json");

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct AreaCondition {
    pub conditions: HashMap<usize, Vec<bool>>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Parent {
    pub monster: Option<usize>,
    pub family: Option<usize>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Monster {
    pub name: String,
    pub rank: usize,
    pub family: usize,
    pub parents: Vec<Vec<Parent>>,
    pub habitats: HashMap<usize, AreaCondition>,
}

fn parse_usize_string_json(json_str: &str) -> HashMap<usize, String> {
    let json: Value = serde_json::from_str(json_str).unwrap();
    let obj: &Map<String, Value> = json.as_object().unwrap();
    let mut data: HashMap<usize, String> = HashMap::new();

    for (key, value) in obj {
        let num_key: usize = key.as_str().parse().unwrap();
        data.insert(num_key, value.as_str().unwrap().to_string());
    }

    data
}

pub fn get_area_data() -> HashMap<usize, String> {
    parse_usize_string_json(AREA_JSON_STR)
}

pub fn get_family_data() -> HashMap<usize, String> {
    parse_usize_string_json(FAMILY_JSON_STR)
}

pub fn get_rank_data() -> HashMap<usize, String> {
    parse_usize_string_json(RANK_JSON_STR)
}

pub fn get_season_data() -> HashMap<usize, String> {
    parse_usize_string_json(SEASON_JSON_STR)
}

pub fn get_monster_data() -> HashMap<usize, Monster> {
    let json: Value = serde_json::from_str(MONSTER_JSON_STR).unwrap();
    let obj: &Map<String, Value> = json.as_object().unwrap();
    let mut data: HashMap<usize, Monster> = HashMap::new();

    for (key, value) in obj {
        let num_key: usize = key.as_str().parse().unwrap();
        let monster = Monster::deserialize(value).unwrap();
        data.insert(num_key, monster);
    }

    data
}
