use crate::packet::{Packet, PacketType};
use crate::{Error, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Join {
    user: String,
    timestamp: i64,
}

impl Join {
    fn new(user: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Self { user, timestamp }
    }
}

impl std::convert::TryFrom<Packet> for Join {
    type Error = Error;

    fn try_from(packet: Packet) -> Result<Self> {
        let packet_contents =
            &String::from_utf8(packet.packet_contents).expect("could not decode as utf8");
        let message: Join = serde_json::from_str(packet_contents)?;
        Ok(message)
    }
}

impl std::convert::TryInto<Packet> for Join {
    type Error = Error;

    fn try_into(self) -> Result<Packet> {
        let packet_contents: Vec<u8> = serde_json::to_string(&self)?.into_bytes();
        let packet_type = PacketType::Join;
        Ok(Packet { packet_type, packet_contents })
    }
}
