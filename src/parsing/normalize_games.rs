use std::collections::HashMap;

use crate::models::Game;

pub fn normalize_games(mut games: Vec<Game>, aliases: &HashMap<String, Vec<String>>) -> Vec<Game> {
    let alias_lookup = create_alias_lookup(aliases);
    games.iter_mut().for_each(|game| {
        game.player_a = convert_if_alias(&game.player_a, &alias_lookup);
        game.player_b = convert_if_alias(&game.player_b, &alias_lookup);
        game.player_x = convert_if_alias(&game.player_x, &alias_lookup);
        game.player_y = convert_if_alias(&game.player_y, &alias_lookup);
    });

    games
}

fn create_alias_lookup(aliases: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    aliases
        .iter()
        .flat_map(|(main_name, alias_list)| {
            alias_list
                .iter()
                .map(move |alias| (alias.clone(), main_name.clone()))
        })
        .collect()
}

fn convert_if_alias(player: &str, alias_lookup: &HashMap<String, String>) -> String {
    alias_lookup
        .get(player)
        .cloned()
        .unwrap_or_else(|| player.to_string())
}

#[cfg(test)]
mod tests {
    use crate::models::Game;

    use super::*;

    #[test]
    fn test_normalize_games() {
        let mut aliases = HashMap::new();
        aliases.insert("Nikhil".to_string(), vec!["Nik".to_string()]);

        let games = vec![Game {
            player_a: "Nik".to_string(),
            player_b: "Alice".to_string(),
            player_x: "Bob".to_string(),
            player_y: "Charlie".to_string(),
            points_ab: 21,
            points_xy: 15,
            date: "08-04-2024".to_string(),
            game_no: 0,
        }];

        let normalized = normalize_games(games, &aliases).get(0).cloned().unwrap();

        assert_eq!(normalized.player_a, "Nikhil".to_string());
        assert_eq!(normalized.player_b, "Alice".to_string());
        assert_eq!(normalized.player_x, "Bob".to_string());
        assert_eq!(normalized.player_y, "Charlie".to_string());
    }
}
