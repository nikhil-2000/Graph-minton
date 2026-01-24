use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    pub date: String,
    pub game_no: u32,
    pub player_a: String,
    pub player_b: String,
    pub points_ab: u8,
    pub player_x: String,
    pub player_y: String,
    pub points_xy: u8,
}

impl Game {
    pub fn new(
        date: String,
        game_no: u32,
        player_a: String,
        player_b: String,
        points_ab: u8,
        player_x: String,
        player_y: String,
        points_xy: u8,
    ) -> Self {
        Self {
            date,
            game_no,
            player_a,
            player_b,
            points_ab,
            player_x,
            player_y,
            points_xy,
        }
    }
}