use std::{collections::HashMap, fs};

use crate::models::Game;

pub fn load_all_games(data_source: &str) -> Vec<Game> {
    let file_paths = match fs::read_dir(data_source) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to read games directory '{}': {}", data_source, e);
            return Vec::new();
        }
    };

    let mut games: Vec<Game> = Vec::new();

    for entry_res in file_paths {
        match entry_res {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    match path.to_str() {
                        Some(path_str) => match load_game(path_str) {
                            Ok(mut sheet_games) => games.append(&mut sheet_games),
                            Err(e) => eprintln!("Failed to load game file '{}': {}", path_str, e),
                        },
                        None => eprintln!("Skipping non-UTF8 path: {:?}", path),
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory entry: {}", e),
        }
    }

    games
}

fn load_game(path: &str) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let mut games = Vec::new();
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        match result {
            Ok(game) => games.push(game),
            Err(e) => {
                eprintln!("{}: failed to parse record: {}", path, e);
            }
        }
    }

    Ok(games)
}

pub fn load_all_aliases(data_source: &str) -> HashMap<String, Vec<String>> {
    let file_paths = match fs::read_dir(data_source) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to read aliases directory '{}': {}", data_source, e);
            return HashMap::new();
        }
    };

    let mut aliases: HashMap<String, Vec<String>> = HashMap::new();
    for entry_res in file_paths {
        match entry_res {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    match path.to_str() {
                        Some(path_str) => match load_alias(path_str) {
                            Ok(alias_list) => match path.file_stem().and_then(|s| s.to_str()) {
                                Some(main_name) => {
                                    aliases.insert(main_name.to_string(), alias_list);
                                }
                                None => {
                                    eprintln!("Skipping alias file with non-UTF8 stem: {:?}", path)
                                }
                            },
                            Err(e) => eprintln!("Failed to load alias file '{}': {}", path_str, e),
                        },
                        None => eprintln!("Skipping non-UTF8 path: {:?}", path),
                    }
                }
            }
            Err(e) => eprintln!("Failed to read directory entry: {}", e),
        }
    }

    aliases
}

fn load_alias(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
}

pub fn create_alias_lookup(aliases: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    aliases
        .iter()
        .flat_map(|(main_name, alias_list)| {
            alias_list
                .iter()
                .map(move |alias| (alias.clone(), main_name.clone()))
        })
        .collect()
}

pub fn normalize_games(mut games: Vec<Game>, alias_lookup: &HashMap<String, String>) -> Vec<Game> {
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
