// dispatcher.rs
// Distribuidor central de entradas.

use super::player_event::PlayerInputEvent;

pub fn dispatch(event: PlayerInputEvent) {
    println!("INPUT RECEIVED:");
    println!(
        "Player {} -> {}",
        event.player_id,
        event.action
    );
}