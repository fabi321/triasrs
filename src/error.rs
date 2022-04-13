use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("network error")]
    NetworkError(#[from] reqwest::Error)
}