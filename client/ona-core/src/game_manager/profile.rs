use serde::Serialize;
use std::path::{Path, PathBuf};

use super::manifest::GameManifest;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameProfile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub developer: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub artwork: Option<String>,
    pub installed: bool,
    pub install_dir: PathBuf,
    pub executable: PathBuf,
    pub working_directory: PathBuf,
    pub arguments: Vec<String>,
    pub input_profile: String,
    pub fullscreen: bool,
    pub resolution: Option<String>,
    pub target_display: Option<String>,
}

impl GameProfile {
    pub fn from_manifest(manifest: GameManifest, install_dir: PathBuf) -> Self {
        let working_directory = manifest
            .execution
            .working_directory
            .as_ref()
            .map(|path| install_dir.join(path))
            .unwrap_or_else(|| install_dir.clone());

        Self {
            id: manifest.identity.id,
            name: manifest.identity.name,
            version: manifest.identity.version,
            developer: manifest.identity.developer,
            description: manifest.presentation.description,
            icon: manifest
                .presentation
                .icon
                .map(|path| asset_url(&install_dir, &path)),
            artwork: manifest
                .presentation
                .artwork
                .map(|path| asset_url(&install_dir, &path)),
            installed: true,
            executable: install_dir.join(manifest.execution.executable),
            working_directory,
            install_dir,
            arguments: manifest.execution.arguments,
            input_profile: manifest.input.profile,
            fullscreen: manifest.display.fullscreen,
            resolution: manifest.display.resolution,
            target_display: manifest.display.target_display,
        }
    }
}

fn asset_url(base_dir: &Path, relative: &str) -> String {
    base_dir.join(relative).to_string_lossy().to_string()
}
