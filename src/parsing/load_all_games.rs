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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_games_happy_path() {
        let result = load_all_games("src/parsing/test_data/games");

        // Check that we loaded games successfully
        assert!(!result.games.is_empty(), "Should load some games");
        assert_eq!(result.games.len(), 3, "Should load 3 valid games");

        // Check Week01 games
        let week01_games: Vec<_> = result
            .games
            .iter()
            .filter(|g| g.date == "2026-01-01")
            .collect();
        assert_eq!(week01_games.len(), 2, "Week01 should have 2 games");

        // Check first game details
        let game1 = &week01_games[0];
        assert_eq!(game1.player_a, "Nikhil");
        assert_eq!(game1.player_b, "Chet");
        assert_eq!(game1.points_ab, 21);
        assert_eq!(game1.player_x, "Chan");
        assert_eq!(game1.player_y, "Bhavin");
        assert_eq!(game1.points_xy, 15);

        // Check Week02 games
        let week02_games: Vec<_> = result
            .games
            .iter()
            .filter(|g| g.date == "2026-01-08")
            .collect();
        assert_eq!(week02_games.len(), 1, "Week02 should have 1 game");
        let game2 = &week02_games[0];
        assert_eq!(game2.player_a, "Bhavin");
        assert_eq!(game2.player_b, "Kishan");
        assert_eq!(game2.points_ab, 21);
        assert_eq!(game2.player_x, "Nikhil");
        assert_eq!(game2.player_y, "Chet");
        assert_eq!(game2.points_xy, 19);
    }

    #[test]
    fn test_load_all_games_invalid_data() {
        let result = load_all_games("src/parsing/test_data/games");

        // Should record Invalid.csv as a failed file since it has invalid data
        assert!(
            result
                .failed_files
                .iter()
                .any(|f| f.contains("Invalid.csv")),
            "Should record Invalid.csv as failed"
        );

        // Should still have loaded the 3 valid games
        assert_eq!(result.games.len(), 3, "Should load valid games and skip invalid");
    }

    #[test]
    fn test_load_all_games_nonexistent_directory() {
        let result = load_all_games("nonexistent/games/directory");

        // Should fail to read directory
        assert!(
            result
                .failed_files
                .contains(&"nonexistent/games/directory".to_string()),
            "Should record nonexistent directory as failed"
        );
        assert!(
            result.games.is_empty(),
            "Should have no games from failed directory"
        );
    }
}