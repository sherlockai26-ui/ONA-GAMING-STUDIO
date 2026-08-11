// manager.rs
// Gestión de sesiones de controladores ONA.

use super::token::SessionToken;

#[derive(Debug, Clone)]
pub struct ControllerSession {
    pub id: String,
    pub token: SessionToken,
    pub players_connected: u8,
}

impl ControllerSession {
    pub fn new() -> Self {
        let token = SessionToken::new();

        Self {
            id: format!("SESSION-{}", token.created_at),
            token,
            players_connected: 0,
        }
    }

    pub fn add_player(&mut self) {
        self.players_connected += 1;
    }
}

pub fn create_session() -> ControllerSession {
    let session = ControllerSession::new();

    println!("Session created:");
    println!("ID: {}", session.id);
    println!("TOKEN: {}", session.token.value);

    session
}
