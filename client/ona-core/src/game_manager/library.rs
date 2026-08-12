use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{
    catalog::GameCatalog,
    errors::{GameManagerError, GameManagerResult},
    manifest::GameManifest,
    profile::GameProfile,
    storage::GameStorage,
};

#[derive(Debug, Clone)]
pub struct GameLibrary {
    storage: GameStorage,
}

impl GameLibrary {
    pub fn new(app_data_dir: impl AsRef<Path>) -> Self {
        Self {
            storage: GameStorage::new(app_data_dir),
        }
    }

    pub fn storage(&self) -> &GameStorage {
        &self.storage
    }

    pub fn list_games(&self) -> GameManagerResult<GameCatalog> {
        self.storage.ensure()?;
        let mut games = Vec::new();

        for entry in std::fs::read_dir(self.storage.games_dir())? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            match self.profile_from_dir(&path) {
                Ok(profile) => games.push(profile),
                Err(error) => eprintln!(
                    "[ONA Game Library] Skipping invalid game at {:?}: {error}",
                    path
                ),
            }
        }

        games.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(GameCatalog::new(games))
    }

    pub fn get_game(&self, game_id: &str) -> GameManagerResult<GameProfile> {
        self.storage.assert_game_id_is_safe(game_id)?;
        let install_dir = self.storage.install_dir(game_id);

        if !install_dir.is_dir() {
            return Err(GameManagerError::NotFound(game_id.to_string()));
        }

        self.profile_from_dir(&install_dir)
    }

    pub fn uninstall_game(&self, game_id: &str) -> GameManagerResult<()> {
        self.storage.ensure()?;
        self.storage.assert_game_id_is_safe(game_id)?;

        let games_dir = self.storage.games_dir();
        let install_dir = self.storage.install_dir(game_id);

        if !install_dir.is_dir() {
            return Err(GameManagerError::NotFound(game_id.to_string()));
        }

        let games_dir = games_dir.canonicalize()?;
        let install_dir = install_dir.canonicalize()?;

        if install_dir == games_dir || !install_dir.starts_with(&games_dir) {
            return Err(GameManagerError::InvalidPath(format!(
                "install directory is not managed by ONA: {}",
                install_dir.display()
            )));
        }

        fs::remove_dir_all(install_dir)?;

        Ok(())
    }

    pub fn profile_from_dir(&self, dir: &Path) -> GameManagerResult<GameProfile> {
        let manifest = GameManifest::read_from_dir(dir)?;
        Ok(GameProfile::from_manifest(manifest, PathBuf::from(dir)))
    }
}
