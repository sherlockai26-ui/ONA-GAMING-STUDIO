// config.rs
// Configuración local de ONA Core.

pub struct CoreConfig {
    pub port: u16,
    pub max_players: u8,
}

impl CoreConfig {
    pub fn default() -> Self {
        Self {
            port: 8080,
            max_players: 10,
        }
    }
}
