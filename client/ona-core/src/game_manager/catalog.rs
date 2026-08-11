use serde::Serialize;

use super::profile::GameProfile;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCatalog {
    pub games: Vec<GameProfile>,
}

impl GameCatalog {
    pub fn new(games: Vec<GameProfile>) -> Self {
        Self { games }
    }

    pub fn list(&self) -> &[GameProfile] {
        &self.games
    }
}
