use crate::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClientConfig {
    pub user: String,
}

impl ClientConfig {
    pub fn load() -> Result<Self> {
        let config = std::fs::read_to_string("./client_config.toml")?;
        let config: ClientConfig = toml::from_str(&config)?;
        Ok(config)
    }
}
