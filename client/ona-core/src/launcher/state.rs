use serde::Serialize;

use crate::runtime::lifecycle::GameLifecycleState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningGameStatus {
    pub game_id: Option<String>,
    pub pid: Option<u32>,
    pub state: GameLifecycleState,
    pub exit_code: Option<i32>,
    pub error: Option<String>,
}

impl RunningGameStatus {
    pub fn idle() -> Self {
        Self {
            game_id: None,
            pid: None,
            state: GameLifecycleState::Idle,
            exit_code: None,
            error: None,
        }
    }
}
