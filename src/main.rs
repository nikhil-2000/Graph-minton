mod data_folders;
mod models;
mod parsing;
mod requests;

use std::collections::HashSet;

use crate::{
    data_folders::{ALIASES_FOLDER, SCORES_FOLDER},
    parsing::{create_alias_lookup, load_all_aliases, load_all_games, normalize_games},
};

fn main() {
    let games_result = load_all_games(SCORES_FOLDER);
    let aliases_result = load_all_aliases(ALIASES_FOLDER);
    
    // Print any failures
    if !games_result.failed_files.is_empty() {
        eprintln!("Failed to load {} game file(s):", games_result.failed_files.len());
        for file in &games_result.failed_files {
            eprintln!("  - {}", file);
        }
    } else {
        println!("Successfully loaded all game files");
    }
    
    if !aliases_result.failed_files.is_empty() {
        eprintln!("Failed to load {} alias file(s):", aliases_result.failed_files.len());
        for file in &aliases_result.failed_files {
            eprintln!("  - {}", file);
        }
    } else {
        println!("Successfully loaded all alias files");
    }
    
    let alias_lookup = create_alias_lookup(&aliases_result.aliases);
    let normalized_games = normalize_games(games_result.games, &alias_lookup);

    let mut unique_player_names: HashSet<String> = HashSet::new();

    for game in &normalized_games {
        unique_player_names.insert(game.player_a.clone());
        unique_player_names.insert(game.player_b.clone());
        unique_player_names.insert(game.player_x.clone());
        unique_player_names.insert(game.player_y.clone());
    }
    
    println!("Total games: {}", normalized_games.len());

    println!("Unique player names:");
    for name in unique_player_names {
        println!("{}", name);
    }
}
