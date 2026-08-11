use std::{
    process::{Child, Command},
    sync::{Arc, Mutex},
};

use crate::{game_manager::profile::GameProfile, runtime::lifecycle::GameLifecycleState};

use super::state::RunningGameStatus;

#[derive(Debug)]
pub struct GameLauncher {
    child: Arc<Mutex<Option<Child>>>,
    status: Arc<Mutex<RunningGameStatus>>,
}

impl Default for GameLauncher {
    fn default() -> Self {
        Self::new()
    }
}

impl GameLauncher {
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(RunningGameStatus::idle())),
        }
    }

    pub fn launch(&self, profile: &GameProfile) -> Result<RunningGameStatus, String> {
        self.refresh_status();

        if self
            .child
            .lock()
            .map_err(|error| error.to_string())?
            .is_some()
        {
            return Err("Another game is already running.".to_string());
        }

        set_status(
            &self.status,
            RunningGameStatus {
                game_id: Some(profile.id.clone()),
                pid: None,
                state: GameLifecycleState::Launching,
                exit_code: None,
                error: None,
            },
        );

        let mut command = Command::new(&profile.executable);
        command
            .current_dir(&profile.working_directory)
            .args(&profile.arguments);

        let child = command.spawn().map_err(|error| {
            set_status(
                &self.status,
                RunningGameStatus {
                    game_id: Some(profile.id.clone()),
                    pid: None,
                    state: GameLifecycleState::Error,
                    exit_code: None,
                    error: Some(error.to_string()),
                },
            );

            error.to_string()
        })?;

        let pid = child.id();
        *self.child.lock().map_err(|error| error.to_string())? = Some(child);

        let running = RunningGameStatus {
            game_id: Some(profile.id.clone()),
            pid: Some(pid),
            state: GameLifecycleState::Running,
            exit_code: None,
            error: None,
        };
        set_status(&self.status, running.clone());

        Ok(running)
    }

    pub fn status(&self) -> RunningGameStatus {
        self.refresh_status();
        self.status
            .lock()
            .map(|status| status.clone())
            .unwrap_or_else(|error| RunningGameStatus {
                game_id: None,
                pid: None,
                state: GameLifecycleState::Error,
                exit_code: None,
                error: Some(error.to_string()),
            })
    }

    pub fn terminate(&self) -> Result<RunningGameStatus, String> {
        set_status_state(&self.status, GameLifecycleState::Exiting);

        let mut child_slot = self.child.lock().map_err(|error| error.to_string())?;
        if let Some(child) = child_slot.as_mut() {
            child.kill().map_err(|error| error.to_string())?;
            let exit = child.wait().map_err(|error| error.to_string())?;
            let mut status = self
                .status
                .lock()
                .map_err(|error| error.to_string())?
                .clone();

            status.pid = None;
            status.state = GameLifecycleState::Idle;
            status.exit_code = exit.code();
            *child_slot = None;
            set_status(&self.status, status.clone());
            Ok(status)
        } else {
            let idle = RunningGameStatus::idle();
            set_status(&self.status, idle.clone());
            Ok(idle)
        }
    }

    fn refresh_status(&self) {
        let Ok(mut child_slot) = self.child.lock() else {
            return;
        };

        let Some(child) = child_slot.as_mut() else {
            return;
        };

        match child.try_wait() {
            Ok(Some(exit)) => {
                let mut status = self
                    .status
                    .lock()
                    .map(|status| status.clone())
                    .unwrap_or_else(|_| RunningGameStatus::idle());

                status.pid = None;
                status.state = GameLifecycleState::Idle;
                status.exit_code = exit.code();
                *child_slot = None;
                set_status(&self.status, status);
            }
            Ok(None) => {}
            Err(error) => {
                let mut status = self
                    .status
                    .lock()
                    .map(|status| status.clone())
                    .unwrap_or_else(|_| RunningGameStatus::idle());

                status.state = GameLifecycleState::Error;
                status.error = Some(error.to_string());
                set_status(&self.status, status);
            }
        }
    }
}

fn set_status(status: &Arc<Mutex<RunningGameStatus>>, next: RunningGameStatus) {
    if let Ok(mut status) = status.lock() {
        *status = next;
    }
}

fn set_status_state(status: &Arc<Mutex<RunningGameStatus>>, state: GameLifecycleState) {
    if let Ok(mut status) = status.lock() {
        status.state = state;
    }
}
