use helix_rs::{HelixDB, HelixDBClient};
use serde_json::json;
use chrono::Utc;

fn create_player_json(name: &str) -> serde_json::Value {
    json!({
        "name": name,
        "aliases": [],
        "isSub": false
    })
}

fn create_with_json(from: &str, to: &str) -> serde_json::Value {
    json!({
        "from": from,
        "to": to,
        "playedOn": Utc::now().to_rfc3339(),
        "order": 1,
    })
}

fn create_against_json(from: &str, to: &str, points_scored: i8, points_conceded: i8) -> serde_json::Value {
    json!({
        "from": from,
        "to": to,
        "playedOn": Utc::now().to_rfc3339(),
        "order": 1,
        "pointsScored": points_scored,
        "pointsConceded": points_conceded
    })
}

fn get_player_id(player_json: &serde_json::Value) -> String {
    player_json["player"]["id"].as_str().unwrap().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HelixDB::new(Some("http://localhost"), Some(6969), None);

    // Create players and collect their IDs
    let names = ["A", "B", "C", "D"];
    let mut ids: Vec<String> = Vec::with_capacity(names.len());
    for name in &names {
        let p_json = create_player_json(name);
        let resp: serde_json::Value = client.query("CreatePlayer", &p_json).await?;
        ids.push(get_player_id(&resp));
    }

    let with_pairs = [(0usize, 1usize), (2, 3)];
    for (i, j) in with_pairs.iter().copied() {
        let req = create_with_json(&ids[i], &ids[j]);
        let _: serde_json::Value = client.query("CreateWith", &req).await?;
    }

    for i in 0..2usize {
        for j in 2..4usize {
            let req1 = create_against_json(&ids[i], &ids[j], 21, 19);
            let req2 = create_against_json(&ids[j], &ids[i], 19, 21);
            let _: serde_json::Value = client.query("CreateAgainst", &req1).await?;
            let _: serde_json::Value = client.query("CreateAgainst", &req2).await?;
        }
    }

    Ok(())
}
