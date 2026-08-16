use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fs, path::Path};

use super::{
    errors::{GameManagerError, GameManagerResult},
    library::GameLibrary,
    manifest::{GameManifest, MANIFEST_FILE_NAME},
    profile::GameProfile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GameInstallAction {
    Install,
    Update,
    Reinstall,
    Downgrade,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInstallPlan {
    pub action: GameInstallAction,
    pub game_id: String,
    pub package_version: String,
    pub installed_version: Option<String>,
    pub already_installed: bool,
}

pub fn import_game_from_dir(
    library: &GameLibrary,
    source_dir: impl AsRef<Path>,
    overwrite: bool,
) -> GameManagerResult<GameProfile> {
    let source_dir = source_dir.as_ref();

    if !source_dir.is_dir() {
        return Err(GameManagerError::InvalidPath(format!(
            "source is not a directory: {}",
            source_dir.display()
        )));
    }

    let manifest = GameManifest::read_from_dir(source_dir)?;
    let game_id = manifest.identity.id.clone();
    let storage = library.storage();
    storage.ensure()?;
    storage.assert_game_id_is_safe(&game_id)?;

    let destination = storage.install_dir(&game_id);
    if destination.exists() {
        if overwrite {
            fs::remove_dir_all(&destination)?;
        } else {
            return Err(GameManagerError::AlreadyInstalled(game_id));
        }
    }

    copy_dir_all(source_dir, &destination)?;

    let installed_manifest = GameManifest::read_from_dir(&destination)?;
    Ok(GameProfile::from_manifest(installed_manifest, destination))
}

pub fn plan_game_install(
    library: &GameLibrary,
    source_dir: impl AsRef<Path>,
) -> GameManagerResult<GameInstallPlan> {
    let source_dir = source_dir.as_ref();

    if !source_dir.is_dir() {
        return Err(GameManagerError::InvalidPath(format!(
            "source is not a directory: {}",
            source_dir.display()
        )));
    }

    let manifest = GameManifest::read_from_dir(source_dir)?;
    let game_id = manifest.identity.id.clone();
    let package_version = manifest.identity.version.clone();
    let storage = library.storage();
    storage.ensure()?;
    storage.assert_game_id_is_safe(&game_id)?;

    let destination = storage.install_dir(&game_id);
    if !destination.exists() {
        return Ok(GameInstallPlan {
            action: GameInstallAction::Install,
            game_id,
            package_version,
            installed_version: None,
            already_installed: false,
        });
    }

    let installed_manifest = GameManifest::read_from_dir(&destination)?;
    let installed_version = installed_manifest.identity.version;
    let action = match compare_versions(&package_version, &installed_version) {
        Ordering::Greater => GameInstallAction::Update,
        Ordering::Equal => GameInstallAction::Reinstall,
        Ordering::Less => GameInstallAction::Downgrade,
    };

    Ok(GameInstallPlan {
        action,
        game_id,
        package_version,
        installed_version: Some(installed_version),
        already_installed: true,
    })
}

pub fn install_or_replace_game_from_dir(
    library: &GameLibrary,
    source_dir: impl AsRef<Path>,
    expected_action: GameInstallAction,
) -> GameManagerResult<GameProfile> {
    let source_dir = source_dir.as_ref();
    let plan = plan_game_install(library, source_dir)?;

    if plan.action != expected_action {
        return Err(GameManagerError::InvalidManifest(format!(
            "install action mismatch: expected {:?}, package requires {:?}",
            expected_action, plan.action
        )));
    }

    if plan.action == GameInstallAction::Install {
        return import_game_from_dir(library, source_dir, false);
    }

    replace_installed_game(library, source_dir, &plan.game_id)
}

fn replace_installed_game(
    library: &GameLibrary,
    source_dir: &Path,
    game_id: &str,
) -> GameManagerResult<GameProfile> {
    let manifest = GameManifest::read_from_dir(source_dir)?;

    if manifest.identity.id != game_id {
        return Err(GameManagerError::InvalidManifest(
            "replacement package game ID changed during install planning".to_string(),
        ));
    }

    let storage = library.storage();
    storage.ensure()?;
    storage.assert_game_id_is_safe(game_id)?;

    let destination = storage.install_dir(game_id);
    if !destination.is_dir() {
        return Err(GameManagerError::NotFound(game_id.to_string()));
    }

    let staging = unique_sibling_path(&destination, "staging");
    let backup = unique_sibling_path(&destination, "backup");

    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }

    if backup.exists() {
        fs::remove_dir_all(&backup)?;
    }

    copy_dir_all(source_dir, &staging)?;
    let staged_manifest = GameManifest::read_from_dir(&staging)?;

    if staged_manifest.identity.id != game_id {
        let _ = fs::remove_dir_all(&staging);
        return Err(GameManagerError::InvalidManifest(
            "replacement package game ID does not match installed game".to_string(),
        ));
    }

    fs::rename(&destination, &backup)?;

    match fs::rename(&staging, &destination) {
        Ok(()) => match GameManifest::read_from_dir(&destination) {
            Ok(installed_manifest) => {
                let profile = GameProfile::from_manifest(installed_manifest, destination);
                let _ = fs::remove_dir_all(&backup);
                Ok(profile)
            }
            Err(error) => {
                let _ = fs::remove_dir_all(&destination);
                let _ = fs::rename(&backup, &destination);
                Err(error)
            }
        },
        Err(error) => {
            let _ = fs::rename(&backup, &destination);
            let _ = fs::remove_dir_all(&staging);
            Err(error.into())
        }
    }
}

fn copy_dir_all(source: &Path, destination: &Path) -> GameManagerResult<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_all(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    if !destination.join(MANIFEST_FILE_NAME).is_file() {
        return Err(GameManagerError::InvalidManifest(format!(
            "copied package is missing {MANIFEST_FILE_NAME}"
        )));
    }

    Ok(())
}

fn unique_sibling_path(path: &Path, suffix: &str) -> std::path::PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| "game".into());

    path.with_file_name(format!("{name}.{suffix}.{timestamp}"))
}

pub fn compare_versions(left: &str, right: &str) -> Ordering {
    let left = VersionKey::parse(left);
    let right = VersionKey::parse(right);
    left.cmp(&right)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct VersionKey {
    core: Vec<u64>,
    prerelease: Vec<VersionIdentifier>,
}

impl VersionKey {
    fn parse(value: &str) -> Self {
        let value = value.trim();
        let without_build = value.split_once('+').map(|parts| parts.0).unwrap_or(value);
        let (core, prerelease) = without_build
            .split_once('-')
            .map(|(core, prerelease)| (core, prerelease))
            .unwrap_or((without_build, ""));

        Self {
            core: core
                .split('.')
                .map(|part| part.parse::<u64>().unwrap_or(0))
                .collect(),
            prerelease: prerelease
                .split('.')
                .filter(|part| !part.is_empty())
                .map(VersionIdentifier::parse)
                .collect(),
        }
    }
}

impl Ord for VersionKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_len = self.core.len().max(other.core.len());

        for index in 0..max_len {
            let left = self.core.get(index).copied().unwrap_or(0);
            let right = other.core.get(index).copied().unwrap_or(0);
            match left.cmp(&right) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }

        match (self.prerelease.is_empty(), other.prerelease.is_empty()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => self.prerelease.cmp(&other.prerelease),
        }
    }
}

impl PartialOrd for VersionKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum VersionIdentifier {
    Number(u64),
    Text(String),
}

impl VersionIdentifier {
    fn parse(value: &str) -> Self {
        value
            .parse::<u64>()
            .map(Self::Number)
            .unwrap_or_else(|_| Self::Text(value.to_ascii_lowercase()))
    }
}

impl Ord for VersionIdentifier {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(left), Self::Number(right)) => left.cmp(right),
            (Self::Number(_), Self::Text(_)) => Ordering::Less,
            (Self::Text(_), Self::Number(_)) => Ordering::Greater,
            (Self::Text(left), Self::Text(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for VersionIdentifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
