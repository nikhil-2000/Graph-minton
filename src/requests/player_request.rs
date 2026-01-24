use serde::Serialize;

#[derive(Serialize)]
pub struct CreatePlayerRequest {
    pub name: String,
    pub aliases: Vec<String>,
    pub is_sub: bool,
}

impl CreatePlayerRequest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            aliases: Vec::new(),
            is_sub: false,
        }
    }
}
