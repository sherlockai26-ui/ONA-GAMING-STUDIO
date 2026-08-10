// catalog.rs
// Catálogo local de juegos MVP.

#[derive(Debug, Clone)]
pub struct GameEntry {
    pub id: String,
    pub name: String,
}

pub struct GameCatalog {
    pub games: Vec<GameEntry>,
}

impl GameCatalog {
    pub fn new() -> Self {
        Self {
            games: vec![
                GameEntry {
                    id: "game001".into(),
                    name: "ONA Test Game".into(),
                }
            ],
        }
    }

    pub fn list(&self) -> &Vec<GameEntry> {
        &self.games
    }
}