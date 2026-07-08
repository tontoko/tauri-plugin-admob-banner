use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("AdMob error: {0}")]
    Admob(String),

    #[error("Not available on this platform")]
    Unsupported,

    #[error("AdMob is not initialized")]
    NotInitialized,

    #[error("UMP consent not yet obtained; call show_privacy_options first")]
    ConsentRequired,

    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
