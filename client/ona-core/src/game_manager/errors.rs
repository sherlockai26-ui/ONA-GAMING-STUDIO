use std::{fmt, io};

#[derive(Debug)]
pub enum GameManagerError {
    Io(io::Error),
    Json(serde_json::Error),
    InvalidManifest(String),
    InvalidPath(String),
    NotFound(String),
    AlreadyInstalled(String),
}

impl fmt::Display for GameManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Json(error) => write!(f, "JSON error: {error}"),
            Self::InvalidManifest(message) => write!(f, "Invalid game manifest: {message}"),
            Self::InvalidPath(message) => write!(f, "Invalid path: {message}"),
            Self::NotFound(id) => write!(f, "Game not found: {id}"),
            Self::AlreadyInstalled(id) => write!(f, "Game is already installed: {id}"),
        }
    }
}

impl std::error::Error for GameManagerError {}

impl From<io::Error> for GameManagerError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for GameManagerError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

pub type GameManagerResult<T> = Result<T, GameManagerError>;
