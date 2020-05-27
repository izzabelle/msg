// modules
mod client;
mod config;
mod server;

use async_std::net::TcpStream;
use futures::io::{ReadHalf, WriteHalf};
use ilmp::encrypt;
use ilmp::Sendable;
use orion::aead;
use ring::{agreement, digest, rand};
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

/// uses ring's agreement to generate key material and key
pub async fn initialize_connection(
    read: &mut ReadHalf<TcpStream>,
    write: &mut WriteHalf<TcpStream>,
) -> Result<aead::SecretKey> {
    // create / send agreement key
    let rng = rand::SystemRandom::new();
    let my_priv_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).expect("ring broke");
    let my_pub_key = my_priv_key.compute_public_key().expect("ring broke");
    let agreement_packet = ilmp::Agreement::new(my_pub_key.as_ref().into());
    ilmp::write(write, agreement_packet, &encrypt::NoEncrypt::new()).await?;

    // receive peer's pub key
    let packet = ilmp::read(read, &encrypt::NoEncrypt::new()).await?.unwrap();
    let agreement_packet = ilmp::Agreement::from_packet(packet)?;
    let peer_pub_key =
        agreement::UnparsedPublicKey::new(&agreement::X25519, agreement_packet.public_key);

    // generate aead key
    agreement::agree_ephemeral(my_priv_key, &peer_pub_key, MsgError::Ring, |key_material| {
        let key_material = digest::digest(&digest::SHA256, key_material.as_ref().into())
            .as_ref()
            .to_vec();
        Ok(aead::SecretKey::from_slice(&key_material)?)
    })
}
