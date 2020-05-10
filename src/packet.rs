// namespacing
use chrono::prelude::*;
use serde::Serialize;

/// structured [packet type byte][four bytes of packet length][contents of packet]
struct NetworkPacket(Vec<u8>);

impl std::convert::TryInto<NetworkPacket> for Packet {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> crate::Result<NetworkPacket> {
        let mut contents: Vec<u8> = Vec::new();

        // packet type byte
        contents.push(self.packet_type as u8);
        // create room for the packet length
        (1..5).for_each(|_| contents.push(0x00));
        // write the rest of the contents
        self.packet_contents
            .iter()
            .for_each(|byte| contents.push(*byte));
        // write the packet len bytes
        let packet_length = ((self.packet_contents.len() + 5) as u32).to_le_bytes();
        (1..5).for_each(|i| contents[i] = packet_length[i - 1]);

        Ok(NetworkPacket(contents))
    }
}

struct Packet {
    packet_type: PacketType,
    packet_contents: Vec<u8>,
}

#[repr(u8)]
enum PacketType {
    NewMessage = 0,
}

#[derive(Serialize)]
struct NewMessage {
    user: String,
    contents: String,
    timestamp: i64,
}

impl NewMessage {
    pub fn new(user: String, contents: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            user,
            contents,
            timestamp,
        }
    }
}

impl std::convert::TryInto<Packet> for NewMessage {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> crate::Result<Packet> {
        let packet_contents: Vec<u8> = serde_json::to_string(&self)?.into_bytes();
        let packet_type = PacketType::NewMessage;
        Ok(Packet {
            packet_type,
            packet_contents,
        })
    }
}
