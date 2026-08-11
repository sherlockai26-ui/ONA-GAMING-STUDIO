use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GameLifecycleState {
    Idle,
    Selecting,
    Preparing,
    Launching,
    Running,
    Paused,
    Exiting,
    Error,
}
