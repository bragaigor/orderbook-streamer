use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptError {
    #[error("missing domain cache for domain {on_domain:?}")]
    DomainCacheLookup { on_domain: String },

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}
