use std::fs;

use crate::{models::Game, parsing::helpers::entry_to_path};

pub struct GamesLoadResult {
    pub games: Vec<Game>,
    pub failed_files: Vec<String>,
}

pub fn load_all_games(data_source: &str) -> GamesLoadResult {
    let entries = match fs::read_dir(data_source) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read games directory '{}': {}", data_source, e);
            return GamesLoadResult {
                games: Vec::new(),
                failed_files: vec![data_source.to_string()],
            };
        }
    };

    let paths = entries
        .filter_map(entry_to_path)
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    let (games, failed_files): (Vec<_>, Vec<_>) = paths
        .iter()
        .map(|path_str| load_games(path_str))
        .unzip();

    GamesLoadResult {
        games: games.into_iter().flatten().collect(),
        failed_files: failed_files.into_iter().flatten().collect(),
    }
}

fn load_games(path: &str) -> (Vec<Game>, Option<String>) {
    match csv::Reader::from_path(path) {
        Ok(mut reader) => match reader.deserialize().collect::<Result<Vec<Game>, _>>() {
            Ok(games) => (games, None),
            Err(e) => {
                eprintln!("Failed to deserialize game file '{}': {}", path, e);
                (Vec::new(), Some(path.to_string()))
            }
        },
        Err(e) => {
            eprintln!("Failed to load game file '{}': {}", path, e);
            (Vec::new(), Some(path.to_string()))
        }
    }
}