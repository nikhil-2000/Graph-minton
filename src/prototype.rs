mod models;
mod requests;

use chrono::Utc;
use helix_rs::{HelixDB, HelixDBClient};

use models::CreatePlayerResponse;
use requests::{CreateAgainstRequest, CreatePlayerRequest, CreateWithRequest};

// This file is just for testing if I could get the db working
#[tokio::main]
async fn prototype() -> Result<(), Box<dyn std::error::Error>> {
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


// Files
// Parse into a struct of games
// For each set of games -> for each game
// -> check if player nodes exist -> add if not
// -> insert with edges
// -> insert against edges