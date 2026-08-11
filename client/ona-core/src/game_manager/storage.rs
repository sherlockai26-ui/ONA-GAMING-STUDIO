use std::{
    fs,
    path::{Path, PathBuf},
};

use super::errors::{GameManagerError, GameManagerResult};

#[derive(Debug, Clone)]
pub struct GameStorage {
    root: PathBuf,
}

impl GameStorage {
    pub fn new(app_data_dir: impl AsRef<Path>) -> Self {
        Self {
            root: app_data_dir.as_ref().join("games"),
        }
    }

    pub fn games_dir(&self) -> &Path {
        &self.root
    }

    pub fn ensure(&self) -> GameManagerResult<()> {
        fs::create_dir_all(&self.root)?;
        Ok(())
    }

    pub fn install_dir(&self, game_id: &str) -> PathBuf {
        self.root.join(game_id)
    }

    pub fn assert_game_id_is_safe(&self, game_id: &str) -> GameManagerResult<()> {
        let install_dir = self.install_dir(game_id);
        if install_dir.parent() != Some(self.root.as_path()) {
            return Err(GameManagerError::InvalidPath(format!(
                "game id escapes storage root: {game_id}"
            )));
        }

        Ok(())
    }
}
