// modules
mod client;
mod config;
mod server;

// re-exports
pub use client::client;
pub use server::server;

// lazy idiot error/result type
pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;
