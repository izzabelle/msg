// modules
mod client;
mod config;
mod server;

use ring::{agreement, rand};
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
}

pub struct AsymmetricKeys {
    pub private: agreement::EphemeralPrivateKey,
    pub public: agreement::PublicKey,
}

impl AsymmetricKeys {
    pub fn generate() -> AsymmetricKeys {
        let rng = rand::SystemRandom::new();
        let private = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
            .expect("failed to create private key");
        let public = private
            .compute_public_key()
            .expect("failed to create public key");
        AsymmetricKeys { private, public }
    }
}
