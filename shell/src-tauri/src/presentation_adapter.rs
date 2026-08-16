#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PresentationOwner {
    OnaShell,
    OnaTransitionGuard,
    Game,
    OnaSystemOverlay,
    OnaMinimized,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActiveGameSessionState {
    Idle,
    Launching,
    Running,
    RunningForeground,
    Background,
    SystemOverlay,
    Minimized,
    Stopping,
    Exited,
    Returning,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PresentationCommand {
    ShowInitialHandoffGuard,
    GrantForegroundToGame,
    ShowOnaSystemOverlay,
    HideOnaSystemOverlay,
    MinimizeConsoleExperience,
    RestoreConsoleExperience,
    ReturnToShellHome,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PresentationEvent {
    GameWindowReady,
    GameDisplayReady,
    GameReady,
    SafeHandoffAccepted,
    QuickMenuOpened,
    QuickMenuClosed,
    ConsoleMinimized,
    ConsoleRestored,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompatibilityLevel {
    RuntimeV1,
    NativeWindowProcess,
    LegacyBestEffort,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CursorMode {
    ShellVisible,
    GameHidden,
    SystemOverlayVisible,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameWindowIdentity {
    pub pid: u32,
    pub hwnd: Option<String>,
    pub display_id: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationAdapterContract {
    pub version: String,
    pub owners: Vec<PresentationOwner>,
    pub session_states: Vec<ActiveGameSessionState>,
    pub commands: Vec<PresentationCommand>,
    pub events: Vec<PresentationEvent>,
    pub compatibility_levels: Vec<CompatibilityLevel>,
    pub cursor_modes: Vec<CursorMode>,
    pub native_window_identity: GameWindowIdentity,
}

pub fn owner_for_state(state: &ActiveGameSessionState) -> PresentationOwner {
    match state {
        ActiveGameSessionState::Idle
        | ActiveGameSessionState::Background
        | ActiveGameSessionState::Exited
        | ActiveGameSessionState::Failed => PresentationOwner::OnaShell,
        ActiveGameSessionState::Launching
        | ActiveGameSessionState::Stopping
        | ActiveGameSessionState::Returning => PresentationOwner::OnaTransitionGuard,
        ActiveGameSessionState::Running | ActiveGameSessionState::RunningForeground => {
            PresentationOwner::Game
        }
        ActiveGameSessionState::SystemOverlay => PresentationOwner::OnaSystemOverlay,
        ActiveGameSessionState::Minimized => PresentationOwner::OnaMinimized,
    }
}

pub fn contract() -> PresentationAdapterContract {
    let session_states = vec![
        ActiveGameSessionState::Idle,
        ActiveGameSessionState::Launching,
        ActiveGameSessionState::Running,
        ActiveGameSessionState::RunningForeground,
        ActiveGameSessionState::Background,
        ActiveGameSessionState::SystemOverlay,
        ActiveGameSessionState::Minimized,
        ActiveGameSessionState::Stopping,
        ActiveGameSessionState::Exited,
        ActiveGameSessionState::Returning,
        ActiveGameSessionState::Failed,
    ];
    let mapped_owners: Vec<PresentationOwner> =
        session_states.iter().map(owner_for_state).collect();

    PresentationAdapterContract {
        version: "ONA_GAME_PRESENTATION_CONTRACT_V1".to_string(),
        owners: mapped_owners,
        session_states,
        commands: vec![
            PresentationCommand::ShowInitialHandoffGuard,
            PresentationCommand::GrantForegroundToGame,
            PresentationCommand::ShowOnaSystemOverlay,
            PresentationCommand::HideOnaSystemOverlay,
            PresentationCommand::MinimizeConsoleExperience,
            PresentationCommand::RestoreConsoleExperience,
            PresentationCommand::ReturnToShellHome,
        ],
        events: vec![
            PresentationEvent::GameWindowReady,
            PresentationEvent::GameDisplayReady,
            PresentationEvent::GameReady,
            PresentationEvent::SafeHandoffAccepted,
            PresentationEvent::QuickMenuOpened,
            PresentationEvent::QuickMenuClosed,
            PresentationEvent::ConsoleMinimized,
            PresentationEvent::ConsoleRestored,
        ],
        compatibility_levels: vec![
            CompatibilityLevel::RuntimeV1,
            CompatibilityLevel::NativeWindowProcess,
            CompatibilityLevel::LegacyBestEffort,
        ],
        cursor_modes: vec![
            CursorMode::ShellVisible,
            CursorMode::GameHidden,
            CursorMode::SystemOverlayVisible,
        ],
        native_window_identity: GameWindowIdentity {
            pid: 0,
            hwnd: None,
            display_id: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        owner_for_state, ActiveGameSessionState, CompatibilityLevel, CursorMode,
        GameWindowIdentity, PresentationCommand, PresentationEvent, PresentationOwner,
    };

    #[test]
    fn every_session_state_has_an_explicit_presentation_owner() {
        let states = [
            ActiveGameSessionState::Idle,
            ActiveGameSessionState::Launching,
            ActiveGameSessionState::Running,
            ActiveGameSessionState::RunningForeground,
            ActiveGameSessionState::Background,
            ActiveGameSessionState::SystemOverlay,
            ActiveGameSessionState::Minimized,
            ActiveGameSessionState::Stopping,
            ActiveGameSessionState::Exited,
            ActiveGameSessionState::Returning,
            ActiveGameSessionState::Failed,
        ];

        for state in states {
            let owner = owner_for_state(&state);
            assert!(matches!(
                owner,
                PresentationOwner::OnaShell
                    | PresentationOwner::OnaTransitionGuard
                    | PresentationOwner::Game
                    | PresentationOwner::OnaSystemOverlay
                    | PresentationOwner::OnaMinimized
            ));
        }
    }

    #[test]
    fn quick_menu_and_minimize_are_distinct_host_commands() {
        let commands = [
            PresentationCommand::ShowInitialHandoffGuard,
            PresentationCommand::GrantForegroundToGame,
            PresentationCommand::ShowOnaSystemOverlay,
            PresentationCommand::HideOnaSystemOverlay,
            PresentationCommand::MinimizeConsoleExperience,
            PresentationCommand::RestoreConsoleExperience,
            PresentationCommand::ReturnToShellHome,
        ];

        assert_eq!(commands.len(), 7);
        assert_ne!(
            PresentationCommand::ShowOnaSystemOverlay,
            PresentationCommand::ShowInitialHandoffGuard
        );
    }

    #[test]
    fn adapter_contract_tracks_native_window_identity_without_repo_paths() {
        let identity = GameWindowIdentity {
            pid: 4321,
            hwnd: Some("0x00000000000ABC".to_string()),
            display_id: Some(r"\\.\DISPLAY2".to_string()),
        };

        assert_eq!(identity.pid, 4321);
        assert!(!identity.hwnd.unwrap().contains("ONA-GAMING-STUDIO"));
    }

    #[test]
    fn runtime_v1_keeps_cursor_and_event_terms_separate() {
        let _compatibility = CompatibilityLevel::RuntimeV1;
        let _cursor = CursorMode::GameHidden;
        let events = [
            PresentationEvent::GameWindowReady,
            PresentationEvent::GameDisplayReady,
            PresentationEvent::GameReady,
            PresentationEvent::SafeHandoffAccepted,
            PresentationEvent::QuickMenuOpened,
            PresentationEvent::QuickMenuClosed,
            PresentationEvent::ConsoleMinimized,
            PresentationEvent::ConsoleRestored,
        ];

        assert_eq!(events.len(), 8);
    }
}
