use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use std::convert::{TryFrom, TryInto};

use packet::{Message, Packet, PacketType};

mod packet;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn server(port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", &port)).await?;
    println!(
        "online as server at: {}:{}",
        listener.local_addr()?.ip(),
        port
    );
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        println!("new stream from: {}", stream.peer_addr()?.ip());
        // handle stream

        // testing
        let packet = Packet::read(&mut stream).await?;
        let message = match packet.packet_type {
            PacketType::Message => Message::try_from(packet),
        };
        println!("{:?}", message);
    }

    Ok(())
}

pub async fn client(port: u16) -> Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!(
        "connection established to: {}:{}",
        stream.peer_addr()?.ip(),
        port
    );

    // testing stuffs
    let message: Packet =
        Message::new("Isabelle".to_owned(), "Hello Server".to_owned()).try_into()?;
    message.write(&mut stream).await?;

    /*let mut buf = vec![0u8; 1024];
    stream.read(&mut buf).await?;
    println!("{}", String::from_utf8_lossy(&mut buf));*/
    Ok(())
}
