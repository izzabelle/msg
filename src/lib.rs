// modules
mod client;
#[allow(dead_code)]
mod packet;
mod server;

// re-exports
pub use client::client;
pub use server::server;

// lazy idiot error/result type
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
