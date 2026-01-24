use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePlayerResponse {
    pub player: Player,
}

#[derive(Deserialize)]
pub struct Player {
    pub aliases: Vec<String>,
    pub id: String,
    pub is_sub: bool,
    pub label: String,
    pub name: String,
}
