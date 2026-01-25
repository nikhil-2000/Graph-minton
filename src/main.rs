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
    let games = load_all_games(SCORES_FOLDER);
    let aliases = load_all_aliases(ALIASES_FOLDER);
    let alias_lookup = create_alias_lookup(&aliases);
    let normalized_games = normalize_games(games, &alias_lookup);

    let mut unique_player_names = HashSet::new();

    for game in &normalized_games {
        println!("{:#?}", game);
        unique_player_names.insert(game.player_a.clone());
        unique_player_names.insert(game.player_b.clone());
        unique_player_names.insert(game.player_x.clone());
        unique_player_names.insert(game.player_y.clone());
    }

    println!("Unique player names:");
    for name in unique_player_names {
        println!("{}", name);
    }
}
