use std::{fs, path::Path};

use super::{
    errors::{GameManagerError, GameManagerResult},
    library::GameLibrary,
    manifest::{GameManifest, MANIFEST_FILE_NAME},
    profile::GameProfile,
};

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
