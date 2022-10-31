use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderbookError {
    // TODO: Future version, add Custom Error handling

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
