// modules
mod client;
mod config;
mod server;

// re-exports
pub use client::client;
pub use server::server;

// lazy idiot error/result type
pub type Result<T> = anyhow::Result<T>;
