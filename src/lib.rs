// modules
mod client;
mod config;
mod server;

// namespacing
use thiserror::Error;

// re-exports
pub use client::client;
pub use server::server;

// lazy idiot error/result type
pub type Result<T> = std::result::Result<T, MsgError>;
#[derive(Debug, Error)]
pub enum MsgError {
    #[error("message protocol error")]
    Ilmp(#[from] ilmp::IlmpError),
    #[error("std::io error")]
    StdIo(#[from] std::io::Error),
    #[error("toml error")]
    Toml(#[from] toml::de::Error),
    #[error("ring fucking broke")]
    Ring,
    #[error("orion error")]
    Orion(#[from] orion::errors::UnknownCryptoError),
}
