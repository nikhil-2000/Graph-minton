use chrono::{DateTime, Utc};
use helix_rs::{HelixDB, HelixDBClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreatePlayerResponse {
    player: Player,
}

#[derive(Deserialize)]
struct Player {
    aliases: Vec<String>,
    id: String,
    is_sub: bool,
    label: String,
    name: String,
}

#[derive(Serialize)]
struct CreatePlayerRequest {
    name: String,
    aliases: Vec<String>,
    is_sub: bool,
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

#[derive(Serialize)]
struct CreateWithRequest {
    from: String,
    to: String,
    played_on: String,
    order: u8,
}

impl CreateWithRequest {
    pub fn new(from: &str, to: &str, played_on: &DateTime<Utc>, order: u8) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            played_on: played_on.to_rfc3339(),
            order,
        }
    }
}

#[derive(Serialize)]
struct CreateAgainstRequest {
    from: String,
    to: String,
    played_on: String,
    order: u8,
    points_scored: u8,
}

impl CreateAgainstRequest {
    pub fn new(
        from: &str,
        to: &str,
        played_on: &DateTime<Utc>,
        order: u8,
        points_scored: u8,
    ) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            played_on: played_on.to_rfc3339(),
            order,
            points_scored,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HelixDB::new(Some("http://localhost"), Some(6969), None);
    let played_on = Utc::now();
    let order: u8 = 1;
    // Create players and collect their IDs
    let names = ["A", "B", "C", "D"];
    let mut ids: Vec<String> = Vec::with_capacity(names.len());
    for name in &names {
        let player_request = CreatePlayerRequest::new(name.to_string());
        let resp_value: serde_json::Value = client.query("CreatePlayer", &player_request).await?;
        let resp: CreatePlayerResponse = serde_json::from_value(resp_value)?;
        ids.push(resp.player.id);
    }

    let with_pairs = [(0usize, 1usize), (2, 3)];
    for (i, j) in with_pairs.iter().copied() {
        let from = &ids[i];
        let to = &ids[j];
        let with_request = CreateWithRequest::new(from, to, &played_on, order);
        let _: serde_json::Value = client.query("CreateWith", &with_request).await?;
    }

    for i in 0..2usize {
        for j in 2..4usize {
            let from = &ids[i];
            let to = &ids[j];
            let against_request1 = CreateAgainstRequest::new(from, to, &played_on, order, 19);
            let against_request2 = CreateAgainstRequest::new(to, from, &played_on, order, 21);
            let _: serde_json::Value = client.query("CreateAgainst", &against_request1).await?;
            let _: serde_json::Value = client.query("CreateAgainst", &against_request2).await?;
        }
    }

    Ok(())
}
