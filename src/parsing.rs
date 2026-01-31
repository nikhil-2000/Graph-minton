use std::{collections::HashMap, fs};

use crate::models::Game;

pub fn load_all_games(data_source: &str) -> Vec<Game> {
    match fs::read_dir(data_source) {
        Ok(entries) => entries
            .filter_map(entry_to_path)
            .filter_map(|path| path.to_str().map(|s| s.to_string()))
            .flat_map(|path_str| {
                load_game(&path_str).unwrap_or_else(|e| {
                    eprintln!("Failed to load game file '{}': {}", path_str, e);
                    Vec::new()
                })
            })
            .collect(),
        Err(e) => {
            eprintln!("Failed to read games directory '{}': {}", data_source, e);
            Vec::new()
        }
    }
}

pub fn load_all_aliases(data_source: &str) -> HashMap<String, Vec<String>> {
    match fs::read_dir(data_source) {
        Ok(entries) => entries
            .filter_map(entry_to_path)
            .filter_map(load_alias_file)
            .collect(),
        Err(e) => {
            eprintln!("Failed to read aliases directory '{}': {}", data_source, e);
            HashMap::new()
        }
    }
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
    games.iter_mut().for_each(|game| {
        game.player_a = normalize_player(&game.player_a, alias_lookup);
        game.player_b = normalize_player(&game.player_b, alias_lookup);
        game.player_x = normalize_player(&game.player_x, alias_lookup);
        game.player_y = normalize_player(&game.player_y, alias_lookup);
    });

    games
}

fn entry_to_path(entry_res: Result<fs::DirEntry, std::io::Error>) -> Option<std::path::PathBuf> {
    entry_res.ok().and_then(|entry| {
        let path = entry.path();
        path.is_file().then_some(path)
    })
}

fn load_game(path: &str) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let rdr = csv::Reader::from_path(path)?;

    Ok(rdr
        .into_deserialize()
        .filter_map(|result| {
            result.map_err(|_| eprintln!("{}: failed to parse record", path)).ok()
        })
        .collect())
}

fn normalize_player(player: &str, alias_lookup: &HashMap<String, String>) -> String {
    alias_lookup.get(player).cloned().unwrap_or_else(|| player.to_string())
}

fn load_alias_file(path: std::path::PathBuf) -> Option<(String, Vec<String>)> {
    let path_str = path.to_str()?.to_string();
    let main_name = path.file_stem()?.to_str()?.to_string();
    
    match fs::read_to_string(&path_str) {
        Ok(contents) => {
            let aliases = contents.lines().map(|line| line.to_string()).collect();
            Some((main_name, aliases))
        }
        Err(e) => {
            eprintln!("Failed to load alias file '{}': {}", path_str, e);
            None
        }
    }
}
