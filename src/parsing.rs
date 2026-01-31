use std::{collections::HashMap, fs};

use crate::models::Game;

pub struct GamesLoadResult {
    pub games: Vec<Game>,
    pub failed_files: Vec<String>,
}

pub struct AliasesLoadResult {
    pub aliases: HashMap<String, Vec<String>>,
    pub failed_files: Vec<String>,
}

pub fn load_all_games(data_source: &str) -> GamesLoadResult {
    match fs::read_dir(data_source) {
        Ok(entries) => {
            let (games, failed_files): (Vec<_>, Vec<_>) = entries
                .filter_map(entry_to_path)
                .filter_map(|path| path.to_str().map(|s| s.to_string()))
                .map(|path_str| {
                    load_game(&path_str)
                        .map(|games| (games, None))
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to load game file '{}': {}", path_str, e);
                            (Vec::new(), Some(path_str))
                        })
                })
                .unzip();

            GamesLoadResult {
                games: games.into_iter().flatten().collect(),
                failed_files: failed_files.into_iter().flatten().collect(),
            }
        }
        Err(e) => {
            eprintln!("Failed to read games directory '{}': {}", data_source, e);
            GamesLoadResult {
                games: Vec::new(),
                failed_files: vec![data_source.to_string()],
            }
        }
    }
}

pub fn load_all_aliases(data_source: &str) -> AliasesLoadResult {
    match fs::read_dir(data_source) {
        Ok(entries) => {
            let (aliases, failed_files): (Vec<_>, Vec<_>) = entries
                .filter_map(entry_to_path)
                .map(|path| load_alias_file(path))
                .map(|result| match result {
                    Ok((name, aliases)) => (Some((name, aliases)), None),
                    Err(path_str) => {
                        eprintln!("Failed to load alias file '{}'", path_str);
                        (None, Some(path_str))
                    }
                })
                .unzip();

            AliasesLoadResult {
                aliases: aliases.into_iter().flatten().collect(),
                failed_files: failed_files.into_iter().flatten().collect(),
            }
        }
        Err(e) => {
            eprintln!("Failed to read aliases directory '{}': {}", data_source, e);
            AliasesLoadResult {
                aliases: HashMap::new(),
                failed_files: vec![data_source.to_string()],
            }
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

fn load_alias_file(path: std::path::PathBuf) -> Result<(String, Vec<String>), String> {
    let path_str = path.to_str().ok_or_else(|| format!("{:?}", path))?.to_string();
    let main_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| path_str.clone())?
        .to_string();

    match fs::read_to_string(&path_str) {
        Ok(contents) => {
            let aliases = contents.lines().map(|line| line.to_string()).collect();
            Ok((main_name, aliases))
        }
        Err(_) => Err(path_str),
    }
}
