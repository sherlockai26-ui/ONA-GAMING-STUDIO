// manager.rs
// Gestión de sesiones de controladores ONA.

use super::token::SessionToken;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

#[derive(Debug, Clone)]
pub struct ControllerSession {
    pub id: String,
    pub token: SessionToken,
    pub players_connected: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerConnectionState {
    Paired,
    Connected,
    DisconnectedTemporary,
}

#[derive(Debug, Clone)]
pub struct PairedController {
    pub session_id: String,
    pub token: String,
    pub device_id: Option<String>,
    pub player_id: u8,
    pub state: ControllerConnectionState,
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

static PAIRING_SESSION: OnceLock<Mutex<ControllerSession>> = OnceLock::new();
static PAIRED_CONTROLLERS: OnceLock<Mutex<HashMap<String, PairedController>>> = OnceLock::new();

fn paired_controllers() -> &'static Mutex<HashMap<String, PairedController>> {
    PAIRED_CONTROLLERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn create_session() -> ControllerSession {
    let session = ControllerSession::new();

    println!("Session created:");
    println!("ID: {}", session.id);
    println!("TOKEN: {}", redact_token(&session.token.value));

    session
}

fn redact_token(value: &str) -> String {
    let prefix: String = value.chars().take(4).collect();
    let suffix: String = value
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    format!("{prefix}****{suffix}")
}

pub fn persistent_pairing_session() -> ControllerSession {
    let session = PAIRING_SESSION.get_or_init(|| Mutex::new(create_session()));

    session
        .lock()
        .expect("controller pairing session lock should not be poisoned")
        .clone()
}

pub fn reset_pairing_session(reason: &str) -> ControllerSession {
    let session = PAIRING_SESSION.get_or_init(|| Mutex::new(create_session()));
    let mut session = session
        .lock()
        .expect("controller pairing session lock should not be poisoned");
    *session = create_session();

    paired_controllers()
        .lock()
        .expect("paired controller lock should not be poisoned")
        .clear();

    println!("[ONA Controller] session expired reason={reason}");
    session.clone()
}

pub fn authenticate_controller(
    session_id: &str,
    token: &str,
    device_id: Option<&str>,
) -> Result<PairedController, String> {
    let session = persistent_pairing_session();

    if session.id != session_id || session.token.value != token {
        println!("[ONA Controller] session expired reason=invalid_credentials");
        return Err("INVALID_CONTROLLER_SESSION".to_string());
    }

    let mut controllers = paired_controllers()
        .lock()
        .map_err(|error| error.to_string())?;

    let controller_key = pairing_key(session_id, device_id);

    if let Some(controller) = controllers.get_mut(&controller_key) {
        controller.state = ControllerConnectionState::Connected;
        println!(
            "[ONA Controller] reconnect authenticated player={} device={}",
            controller.player_id,
            controller.device_id.as_deref().unwrap_or("legacy")
        );
        return Ok(controller.clone());
    }

    let player_id = (controllers.len() as u8).saturating_add(1);
    let controller = PairedController {
        session_id: session_id.to_string(),
        token: token.to_string(),
        device_id: device_id.map(str::to_string),
        player_id,
        state: ControllerConnectionState::Connected,
    };

    controllers.insert(controller_key, controller.clone());
    println!(
        "[ONA Controller] paired session={session_id} player={player_id} device={}",
        controller.device_id.as_deref().unwrap_or("legacy")
    );

    Ok(controller)
}

pub fn mark_controller_disconnected(
    session_id: &str,
    device_id: Option<&str>,
) -> Option<PairedController> {
    let mut controllers = paired_controllers().lock().ok()?;
    let controller = controllers.get_mut(&pairing_key(session_id, device_id))?;
    controller.state = ControllerConnectionState::DisconnectedTemporary;
    println!(
        "[ONA Controller] websocket disconnected player={} device={}",
        controller.player_id,
        controller.device_id.as_deref().unwrap_or("legacy")
    );
    println!("[ONA Controller] pairing retained");
    println!("[ONA Controller] waiting for reconnect");

    Some(controller.clone())
}

fn pairing_key(session_id: &str, device_id: Option<&str>) -> String {
    match device_id.filter(|id| !id.trim().is_empty()) {
        Some(id) => format!("{session_id}::{id}"),
        None => session_id.to_string(),
    }
}

#[cfg(test)]
pub fn clear_persistent_pairings_for_tests() {
    if let Some(session) = PAIRING_SESSION.get() {
        *session
            .lock()
            .expect("controller pairing session lock should not be poisoned") =
            ControllerSession::new();
    }

    paired_controllers()
        .lock()
        .expect("paired controller lock should not be poisoned")
        .clear();
}

#[cfg(test)]
mod tests {
    use super::{
        authenticate_controller, clear_persistent_pairings_for_tests, mark_controller_disconnected,
        persistent_pairing_session, ControllerConnectionState,
    };

    #[test]
    fn websocket_drop_retains_pairing_session() {
        clear_persistent_pairings_for_tests();
        let session = persistent_pairing_session();
        let paired = authenticate_controller(&session.id, &session.token.value, None)
            .expect("controller should pair");

        let disconnected = mark_controller_disconnected(&session.id, None)
            .expect("paired controller should be marked disconnected");

        assert_eq!(paired.player_id, disconnected.player_id);
        assert_eq!(
            disconnected.state,
            ControllerConnectionState::DisconnectedTemporary
        );
        assert_eq!(persistent_pairing_session().id, session.id);
        assert_eq!(
            persistent_pairing_session().token.value,
            session.token.value
        );
    }

    #[test]
    fn reconnect_restores_same_player_id() {
        clear_persistent_pairings_for_tests();
        let session = persistent_pairing_session();
        let first = authenticate_controller(&session.id, &session.token.value, None)
            .expect("controller should pair");
        let _ = mark_controller_disconnected(&session.id, None);

        let resumed = authenticate_controller(&session.id, &session.token.value, None)
            .expect("controller should resume");

        assert_eq!(resumed.player_id, first.player_id);
        assert_eq!(resumed.state, ControllerConnectionState::Connected);
    }

    #[test]
    fn invalid_token_is_rejected_without_replacing_pairing_session() {
        clear_persistent_pairings_for_tests();
        let session = persistent_pairing_session();

        assert!(authenticate_controller(&session.id, "WRONG", None).is_err());
        assert_eq!(persistent_pairing_session().id, session.id);
        assert_eq!(
            persistent_pairing_session().token.value,
            session.token.value
        );
    }

    #[test]
    fn device_id_gets_independent_player_slot_with_same_pairing() {
        clear_persistent_pairings_for_tests();
        let session = persistent_pairing_session();

        let first =
            authenticate_controller(&session.id, &session.token.value, Some("ona-device-a"))
                .expect("first device should pair");
        let second =
            authenticate_controller(&session.id, &session.token.value, Some("ona-device-b"))
                .expect("second device should pair");

        assert_eq!(first.player_id, 1);
        assert_eq!(second.player_id, 2);
    }

    #[test]
    fn reconnect_by_device_id_restores_same_player_slot() {
        clear_persistent_pairings_for_tests();
        let session = persistent_pairing_session();
        let first =
            authenticate_controller(&session.id, &session.token.value, Some("ona-device-a"))
                .expect("device should pair");
        let _ = mark_controller_disconnected(&session.id, Some("ona-device-a"));

        let resumed =
            authenticate_controller(&session.id, &session.token.value, Some("ona-device-a"))
                .expect("device should resume");

        assert_eq!(resumed.player_id, first.player_id);
    }
}
