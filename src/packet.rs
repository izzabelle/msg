// namespacing
use crate::Result;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures_util::io::ReadHalf;
use std::convert::TryInto;

mod join;
pub use join::Join;
mod message;
pub use message::Message;

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

pub trait Sendable {
    fn to_packet(self) -> Packet;
    fn from_packet(packet: Packet) -> Self;
}

/// contains data to be turned into a network packet or into a more specific packet
pub struct Packet {
    pub packet_type: PacketType,
    packet_contents: Vec<u8>,
}

impl Packet {
    /// create a new packet
    pub fn new(packet_type: PacketType, packet_contents: Vec<u8>) -> Self {
        Self { packet_type, packet_contents }
    }

    /// read a packet from a tcpstream
    pub async fn read(stream: &mut ReadHalf<TcpStream>) -> Result<Option<Packet>> {
        let mut info_buf = [0u8; 5];
        let check = stream.read(&mut info_buf).await?;
        if check == 0 {
            return Ok(None);
        }

        let packet_type = PacketType::from_u8(info_buf[0]).unwrap();

        let length = u32::from_le_bytes(info_buf[1..5].try_into().unwrap()) as usize;

        let mut contents: Vec<u8> = vec![0; length];
        stream.read(&mut contents).await?;

        Ok(Some(Packet::new(packet_type, contents)))
    }

    /// write a packet to the tcpstream
    pub async fn write(self, stream: &mut TcpStream) -> Result<()> {
        let network_packet: NetworkPacket = self.into();
        stream.write(&network_packet.0).await?;
        Ok(())
    }
}

/// represent the specific packet type
#[repr(u8)]
pub enum PacketType {
    Message = 0,
    Join = 1,
}

impl PacketType {
    /// returns the PacketType if the u8 is a valid packet type
    pub fn from_u8(packet_type: u8) -> Option<Self> {
        match packet_type {
            0 => Some(Self::Message),
            _ => None,
        }
    }
}
