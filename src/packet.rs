use chrono::prelude::*;

struct NetworkPacket(String);

struct Packet {
    packet_type: PacketType,
    packet_length: u64,
    packet_contents: Vec<u8>,
}

#[repr(u8)]
enum PacketType {
    NewMessage = 0,
}

struct NewMessage {
    user: String,
    contents: String,
    timestamp: i64,
}

impl NewMessage {
    pub fn new(user: String, contents: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Self { user, contents, timestamp }
    }
}
