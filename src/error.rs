use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("network error")]
    NetworkError(#[from] reqwest::Error),
    #[error("response xml parsing error")]
    XMLResponseError(#[from] sxd_document::parser::Error),
}