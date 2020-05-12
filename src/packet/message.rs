// namespacing
use crate::packet::{Packet, PacketType};
use crate::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// a Message
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    user: String,
    contents: String,
    timestamp: i64,
}

impl Message {
    /// create a new message
    pub fn new(user: String, contents: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Self { user, contents, timestamp }
    }
}

impl crate::packet::Sendable for Message {
    fn to_packet(self) -> Result<Packet> {
        let packet_contents: Vec<u8> = serde_json::to_string(&self)?.into_bytes();
        let packet_type = PacketType::Message;
        Ok(Packet { packet_type, packet_contents })
    }

    fn from_packet(packet: Packet) -> Result<Self> {
        let packet_contents =
            &String::from_utf8(packet.packet_contents).expect("could not decode as utf8");
        let message: Message = serde_json::from_str(packet_contents)?;
        Ok(message)
    }
}
