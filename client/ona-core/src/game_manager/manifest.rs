use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use super::errors::{GameManagerError, GameManagerResult};

pub const MANIFEST_FILE_NAME: &str = "game.json";
pub const EXPERIMENTAL_MANIFEST_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameManifest {
    #[serde(default = "default_manifest_version")]
    pub manifest_version: u16,
    pub identity: GameIdentity,
    pub presentation: GamePresentation,
    pub execution: GameExecution,
    #[serde(default)]
    pub requirements: GameRequirements,
    #[serde(default)]
    pub display: GameDisplay,
    #[serde(default)]
    pub input: GameInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameIdentity {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub developer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePresentation {
    #[serde(default)]
    pub icon: Option<String>,
    pub description: String,
    #[serde(default)]
    pub artwork: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameExecution {
    pub executable: String,
    #[serde(default)]
    pub working_directory: Option<String>,
    #[serde(default)]
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRequirements {
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub architecture: Option<String>,
    #[serde(default)]
    pub memory: Option<String>,
    #[serde(default)]
    pub gpu: Option<String>,
    #[serde(default)]
    pub cpu: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDisplay {
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub target_display: Option<String>,
}

impl Default for GameDisplay {
    fn default() -> Self {
        Self {
            fullscreen: true,
            resolution: None,
            target_display: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInput {
    #[serde(default = "default_input_profile")]
    pub profile: String,
}

impl Default for GameInput {
    fn default() -> Self {
        Self {
            profile: default_input_profile(),
        }
    }
}

fn default_manifest_version() -> u16 {
    EXPERIMENTAL_MANIFEST_VERSION
}

fn default_input_profile() -> String {
    "ona-standard-controller-v1".to_string()
}

impl GameManifest {
    pub fn read_from_dir(game_dir: &Path) -> GameManagerResult<Self> {
        let manifest_path = game_dir.join(MANIFEST_FILE_NAME);
        let manifest = fs::read_to_string(&manifest_path)?;
        let manifest = serde_json::from_str::<Self>(&manifest)?;
        manifest.validate(game_dir)?;
        Ok(manifest)
    }

    pub fn validate(&self, base_dir: &Path) -> GameManagerResult<()> {
        validate_id(&self.identity.id)?;
        validate_required("identity.name", &self.identity.name)?;
        validate_required("identity.version", &self.identity.version)?;
        validate_required("presentation.description", &self.presentation.description)?;
        validate_relative_path("execution.executable", &self.execution.executable)?;

        let executable_path = base_dir.join(&self.execution.executable);
        if !executable_path.is_file() {
            return Err(GameManagerError::InvalidManifest(format!(
                "execution.executable does not exist: {}",
                self.execution.executable
            )));
        }

        if let Some(working_directory) = &self.execution.working_directory {
            validate_relative_path("execution.workingDirectory", working_directory)?;
            if !base_dir.join(working_directory).is_dir() {
                return Err(GameManagerError::InvalidManifest(format!(
                    "execution.workingDirectory does not exist: {working_directory}"
                )));
            }
        }

        if let Some(icon) = &self.presentation.icon {
            validate_relative_path("presentation.icon", icon)?;
        }

        if let Some(artwork) = &self.presentation.artwork {
            validate_relative_path("presentation.artwork", artwork)?;
        }

        Ok(())
    }
}

fn validate_required(field: &str, value: &str) -> GameManagerResult<()> {
    if value.trim().is_empty() {
        Err(GameManagerError::InvalidManifest(format!(
            "{field} is required"
        )))
    } else {
        Ok(())
    }
}

fn validate_id(id: &str) -> GameManagerResult<()> {
    validate_required("identity.id", id)?;

    let is_valid = id.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || character == '-'
            || character == '_'
            || character == '.'
    });

    if is_valid {
        Ok(())
    } else {
        Err(GameManagerError::InvalidManifest(
            "identity.id may only contain lowercase letters, numbers, '.', '_' and '-'".to_string(),
        ))
    }
}

fn validate_relative_path(field: &str, value: &str) -> GameManagerResult<()> {
    let path = Path::new(value);
    if value.trim().is_empty() || path.is_absolute() || value.contains("..") {
        return Err(GameManagerError::InvalidManifest(format!(
            "{field} must be a relative path inside the game package"
        )));
    }

    Ok(())
}
