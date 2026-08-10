// player_event.rs
// Evento interno de entrada.
// No sabe si viene de teclado, celular o control físico.

#[derive(Debug, Clone)]
pub struct PlayerInputEvent {
    pub player_id: u8,
    pub action: String,
}

impl PlayerInputEvent {
    pub fn new(player_id: u8, action: &str) -> Self {
        Self {
            player_id,
            action: action.to_string(),
        }
    }
}