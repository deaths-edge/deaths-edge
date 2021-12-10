use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameId(u64);

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterDetails {
    id: PlayerId,
    address: SocketAddr,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerId(u64);

#[derive(Debug, Deserialize, Serialize)]
pub struct GameDetails {
    game_id: GameId,
    team_a: Vec<CharacterDetails>,
    team_b: Vec<CharacterDetails>,
}
