use crate::packet::{Packet, PacketType};
use crate::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Join {
    user: String,
    timestamp: i64,
}

impl Join {
    pub fn new(user: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Self { user, timestamp }
    }
}

impl crate::packet::Sendable for Join {
    fn to_packet(self) -> Result<Packet> {
        let packet_contents: Vec<u8> = serde_json::to_string(&self)?.into_bytes();
        let packet_type = PacketType::Join;
        Ok(Packet { packet_type, packet_contents })
    }

    fn from_packet(packet: Packet) -> Result<Self> {
        let packet_contents =
            &String::from_utf8(packet.packet_contents).expect("could not decode as utf8");
        let join: Join = serde_json::from_str(packet_contents)?;
        Ok(join)
    }
}
