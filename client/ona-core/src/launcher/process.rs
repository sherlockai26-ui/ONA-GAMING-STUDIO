use std::{
    process::{Child, Command},
    sync::{Arc, Mutex},
};

use crate::{game_manager::profile::GameProfile, runtime::lifecycle::GameLifecycleState};

use super::{ownership::ProcessOwnership, state::RunningGameStatus};

pub const ONA_RUNTIME_PROTOCOL_VERSION: &str = "1";

#[derive(Debug, Clone)]
pub struct OnaGameRuntimeContext {
    pub input_host: String,
    pub input_port: u16,
    pub lifecycle_host: String,
    pub lifecycle_port: u16,
    pub player_id: Option<u8>,
    pub display_mode: String,
    pub display_id: String,
    pub display_name: Option<String>,
    pub display_x: i32,
    pub display_y: i32,
    pub display_width: Option<u32>,
    pub display_height: Option<u32>,
    pub display_scale_factor: f64,
    pub display_target: Option<String>,
}

#[derive(Debug)]
pub struct GameLauncher {
    child: Arc<Mutex<Option<Child>>>,
    ownership: Arc<Mutex<Option<ProcessOwnership>>>,
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
            ownership: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(RunningGameStatus::idle())),
        }
    }

    pub fn launch(&self, profile: &GameProfile) -> Result<RunningGameStatus, String> {
        self.launch_internal(profile, None)
    }

    pub fn launch_with_runtime(
        &self,
        profile: &GameProfile,
        runtime: OnaGameRuntimeContext,
    ) -> Result<RunningGameStatus, String> {
        self.launch_internal(profile, Some(runtime))
    }

    fn launch_internal(
        &self,
        profile: &GameProfile,
        runtime: Option<OnaGameRuntimeContext>,
    ) -> Result<RunningGameStatus, String> {
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

        if let Some(runtime) = runtime {
            apply_ona_runtime_environment(&mut command, &runtime);
        }

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

        let ownership = ProcessOwnership::attach(&child);
        let pid = child.id();
        *self.child.lock().map_err(|error| error.to_string())? = Some(child);
        *self.ownership.lock().map_err(|error| error.to_string())? = Some(ownership);

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
            if let Ok(mut ownership) = self.ownership.lock() {
                *ownership = None;
            }
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
                if let Ok(mut ownership) = self.ownership.lock() {
                    *ownership = None;
                }
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
                status.pid = None;
                *child_slot = None;
                if let Ok(mut ownership) = self.ownership.lock() {
                    *ownership = None;
                }
                set_status(&self.status, status);
            }
        }
    }
}

impl Drop for GameLauncher {
    fn drop(&mut self) {
        if let Ok(mut child_slot) = self.child.lock() {
            if let Some(child) = child_slot.as_mut() {
                let _ = child.kill();
                let _ = child.wait();
            }

            *child_slot = None;
        }

        if let Ok(mut ownership) = self.ownership.lock() {
            *ownership = None;
        }
    }
}

fn apply_ona_runtime_environment(command: &mut Command, runtime: &OnaGameRuntimeContext) {
    command
        .env("ONA_INPUT_HOST", &runtime.input_host)
        .env("ONA_INPUT_PORT", runtime.input_port.to_string())
        .env("ONA_LIFECYCLE_HOST", &runtime.lifecycle_host)
        .env("ONA_LIFECYCLE_PORT", runtime.lifecycle_port.to_string())
        .env("ONA_RUNTIME", "1")
        .env("ONA_PROTOCOL_VERSION", ONA_RUNTIME_PROTOCOL_VERSION)
        .env("ONA_DISPLAY_MODE", &runtime.display_mode)
        .env("ONA_DISPLAY_ID", &runtime.display_id)
        .env("ONA_DISPLAY_X", runtime.display_x.to_string())
        .env("ONA_DISPLAY_Y", runtime.display_y.to_string())
        .env(
            "ONA_DISPLAY_SCALE_FACTOR",
            runtime.display_scale_factor.to_string(),
        );

    if let Some(name) = &runtime.display_name {
        command.env("ONA_DISPLAY_NAME", name);
    }

    if let Some(width) = runtime.display_width {
        command.env("ONA_DISPLAY_WIDTH", width.to_string());
    }

    if let Some(height) = runtime.display_height {
        command.env("ONA_DISPLAY_HEIGHT", height.to_string());
    }

    if let Some(target) = &runtime.display_target {
        command.env("ONA_DISPLAY_TARGET", target);
    }

    if let Some(player_id) = runtime.player_id {
        command.env("ONA_PLAYER_ID", player_id.to_string());
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
