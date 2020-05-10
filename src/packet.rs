// namespacing
use crate::Result;
use async_std::net::TcpStream;
use async_std::prelude::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// structured [packet type byte][four bytes of packet length][contents of packet]
pub struct NetworkPacket(Vec<u8>);

impl std::convert::Into<NetworkPacket> for Packet {
    fn into(self) -> NetworkPacket {
        let mut contents: Vec<u8> = Vec::new();

        // packet type byte
        contents.push(self.packet_type as u8);
        // write the packet length
        let contents_length = self.packet_contents.len() as u32;
        contents.extend_from_slice(&contents_length.to_le_bytes());
        // write the rest of the contents
        contents.extend_from_slice(&self.packet_contents);
        NetworkPacket(contents)
    }
}

pub struct Packet {
    pub packet_type: PacketType,
    packet_contents: Vec<u8>,
}

impl Packet {
    pub fn new(packet_type: PacketType, packet_contents: Vec<u8>) -> Self {
        Self {
            packet_type,
            packet_contents,
        }
    }

    pub async fn read(stream: &mut TcpStream) -> Result<Packet> {
        let mut info_buf = [0u8; 5];
        stream.read(&mut info_buf).await?;
        let packet_type = PacketType::from_u8(info_buf[0]).unwrap();

        let length = u32::from_le_bytes(info_buf[1..5].try_into().unwrap()) as usize;

        let mut contents: Vec<u8> = vec![0; length];
        stream.read(&mut contents).await?;

        Ok(Packet::new(packet_type, contents))
    }

    pub async fn write(self, stream: &mut TcpStream) -> Result<()> {
        let network_packet: NetworkPacket = self.into();
        let _ = stream.write(&network_packet.0).await?;
        Ok(())
    }
}

#[repr(u8)]
pub enum PacketType {
    Message = 0,
}

impl PacketType {
    pub fn from_u8(packet_type: u8) -> Option<Self> {
        match packet_type {
            0 => Some(Self::Message),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    user: String,
    contents: String,
    timestamp: i64,
}

impl Message {
    pub fn new(user: String, contents: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            user,
            contents,
            timestamp,
        }
    }
}

impl std::convert::TryFrom<Packet> for Message {
    type Error = Box<dyn std::error::Error>;

    fn try_from(packet: Packet) -> crate::Result<Self> {
        let packet_contents = &String::from_utf8(packet.packet_contents)?;
        let message: Message = serde_json::from_str(packet_contents)?;
        Ok(message)
    }
}

impl std::convert::TryInto<Packet> for Message {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> crate::Result<Packet> {
        let packet_contents: Vec<u8> = serde_json::to_string(&self)?.into_bytes();
        let packet_type = PacketType::Message;
        Ok(Packet {
            packet_type,
            packet_contents,
        })
    }
}
