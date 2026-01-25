use std::{collections::HashMap, fs};

use crate::models::Game;

pub fn load_all_games(data_source: &str) -> Vec<Game> {
    let file_paths = fs::read_dir(data_source).unwrap();
    let mut games: Vec<Game> = Vec::new();

    for entry in file_paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let sheet_games = load_game(path.to_str().unwrap());
            games.extend(sheet_games.unwrap());
        }
    }

    games
}

fn load_game(path: &str) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let mut games: Vec<Game> = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let game: Game = result?;

        games.push(game);
    }

    Ok(games)
}

pub fn load_all_aliases(data_source: &str) -> HashMap<String, Vec<String>> {
    let file_paths = fs::read_dir(data_source).unwrap();

    let mut aliases: HashMap<String, Vec<String>> = HashMap::new();
    for entry in file_paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let alias_list = load_alias(path.to_str().unwrap()).unwrap();
            let main_name = path.file_stem().unwrap().to_str().unwrap().to_string();
            aliases.insert(main_name, alias_list);
        }
    }
    
    aliases
}

fn load_alias(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut aliases: Vec<String> = Vec::new();
    let rdr = fs::read_to_string(path)?;

    for result in rdr.lines() {
        aliases.push(result.to_string());
    }

    Ok(aliases)
}

pub fn create_alias_lookup(aliases: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut lookup: HashMap<String, String> = HashMap::new();

    for (main_name, alias_list) in aliases {
        for alias in alias_list {
            lookup.insert(alias.to_string(), main_name.to_string());
        }
    }

    lookup
}

pub fn normalize_games(
    mut games: Vec<Game>,
    alias_lookup: &HashMap<String, String>,
) -> Vec<Game> {
    for game in &mut games {
        game.player_a = alias_lookup
            .get(&game.player_a)
            .cloned()
            .unwrap_or_else(|| game.player_a.clone());
        
        game.player_b = alias_lookup
            .get(&game.player_b)
            .cloned()
            .unwrap_or_else(|| game.player_b.clone());
        
        game.player_x = alias_lookup
            .get(&game.player_x)
            .cloned()
            .unwrap_or_else(|| game.player_x.clone());
        
        game.player_y = alias_lookup
            .get(&game.player_y)
            .cloned()
            .unwrap_or_else(|| game.player_y.clone());
    }

    games
}