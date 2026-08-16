use serde::Serialize;
use std::path::{Path, PathBuf};

use super::{
    importer::{plan_game_install, GameInstallAction},
    library::GameLibrary,
    manifest::GameManifest,
};

const ONA_LIBRARY_DIR: &str = "ONA Library";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationSource {
    pub id: String,
    pub name: String,
    pub root: PathBuf,
    pub library_path: PathBuf,
    pub kind: InstallationSourceKind,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InstallationSourceKind {
    ExternalStorage,
    LocalFolder,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannedGamePackage {
    pub package_id: String,
    pub source_id: String,
    pub source_name: String,
    pub source_kind: InstallationSourceKind,
    pub package_path: PathBuf,
    pub name: String,
    pub game_id: String,
    pub version: String,
    pub developer: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub status: PackageScanStatus,
    pub already_installed: bool,
    pub install_action: GameInstallAction,
    pub installed_version: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidGamePackage {
    pub source_id: String,
    pub source_name: String,
    pub package_path: PathBuf,
    pub error: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageScanReport {
    pub sources: Vec<InstallationSource>,
    pub games: Vec<ScannedGamePackage>,
    pub invalid_packages: Vec<InvalidGamePackage>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PackageScanStatus {
    ReadyToInstall,
    AlreadyInstalled,
}

pub fn scan_external_game_packages(library: &GameLibrary) -> PackageScanReport {
    scan_game_packages(library, discover_installation_sources())
}

pub fn scan_game_packages(
    library: &GameLibrary,
    sources: Vec<InstallationSource>,
) -> PackageScanReport {
    let mut games = Vec::new();
    let mut invalid_packages = Vec::new();

    for source in &sources {
        let Ok(entries) = std::fs::read_dir(&source.library_path) else {
            continue;
        };

        for entry in entries.flatten() {
            let package_path = entry.path();
            if !package_path.is_dir() || !package_path.join("game.json").is_file() {
                continue;
            }

            match GameManifest::read_from_dir(&package_path) {
                Ok(manifest) => {
                    let plan = plan_game_install(library, &package_path).ok();
                    let already_installed = plan
                        .as_ref()
                        .map(|plan| plan.already_installed)
                        .unwrap_or(false);

                    games.push(ScannedGamePackage {
                        package_id: format!("{}:{}", source.id, manifest.identity.id),
                        source_id: source.id.clone(),
                        source_name: source.name.clone(),
                        source_kind: source.kind.clone(),
                        package_path: package_path.clone(),
                        name: manifest.identity.name,
                        game_id: manifest.identity.id,
                        version: manifest.identity.version,
                        developer: manifest.identity.developer,
                        description: manifest.presentation.description,
                        icon: manifest
                            .presentation
                            .icon
                            .map(|icon| package_path.join(icon).to_string_lossy().to_string()),
                        status: if already_installed {
                            PackageScanStatus::AlreadyInstalled
                        } else {
                            PackageScanStatus::ReadyToInstall
                        },
                        already_installed,
                        install_action: plan
                            .as_ref()
                            .map(|plan| plan.action)
                            .unwrap_or(GameInstallAction::Install),
                        installed_version: plan.and_then(|plan| plan.installed_version),
                    });
                }
                Err(error) => invalid_packages.push(InvalidGamePackage {
                    source_id: source.id.clone(),
                    source_name: source.name.clone(),
                    package_path,
                    error: error.to_string(),
                }),
            }
        }
    }

    games.sort_by(|left, right| {
        left.source_name
            .cmp(&right.source_name)
            .then(left.name.cmp(&right.name))
    });

    PackageScanReport {
        sources,
        games,
        invalid_packages,
    }
}

pub fn discover_installation_sources() -> Vec<InstallationSource> {
    candidate_drive_roots()
        .into_iter()
        .filter_map(|root| source_from_root(&root))
        .collect()
}

fn source_from_root(root: &Path) -> Option<InstallationSource> {
    let library_path = root.join(ONA_LIBRARY_DIR);
    if !library_path.is_dir() {
        return None;
    }

    let id = root
        .to_string_lossy()
        .trim_end_matches('\\')
        .trim_end_matches('/')
        .to_string();

    Some(InstallationSource {
        name: format!("USB DRIVE {id}"),
        id,
        root: root.to_path_buf(),
        library_path,
        kind: InstallationSourceKind::ExternalStorage,
    })
}

#[cfg(windows)]
fn candidate_drive_roots() -> Vec<PathBuf> {
    ('A'..='Z')
        .map(|letter| PathBuf::from(format!("{letter}:\\")))
        .filter(|path| path.is_dir())
        .collect()
}

#[cfg(not(windows))]
fn candidate_drive_roots() -> Vec<PathBuf> {
    ["/media", "/mnt", "/Volumes"]
        .into_iter()
        .flat_map(|root| {
            std::fs::read_dir(root)
                .into_iter()
                .flat_map(|entries| entries.flatten().map(|entry| entry.path()))
                .collect::<Vec<_>>()
        })
        .collect()
}
