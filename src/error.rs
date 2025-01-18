use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Octocrab(#[from] octocrab::Error),
    #[error("{0}")]
    Scraper(String),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Fs(#[from] std::io::Error),
    #[error("Could not find {0} to download.")]
    VersionNotFound(String),
    #[error("{0} is not a valid Python version")]
    InvalidVersion(String),
    #[error("Could not parse version and release_tag from {0}.")]
    ParseAsset(String),
    #[error("{0} is not supported.")]
    Platform(String),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
}
