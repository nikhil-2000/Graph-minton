use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "GameNo")]
    pub game_no: u32,
    #[serde(rename = "A")]
    pub player_a: String,
    #[serde(rename = "B")]
    pub player_b: String,
    #[serde(rename = "PtsAB")]
    pub points_ab: u8,
    #[serde(rename = "X")]
    pub player_x: String,
    #[serde(rename = "Y")]
    pub player_y: String,
    #[serde(rename = "PtsXY")]
    pub points_xy: u8,
}