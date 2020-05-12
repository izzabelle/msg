// namespacing
use crate::config::ClientConfig as Config;
use crate::packet::{Join, Message, Packet, Sendable};
use crate::Result;
use async_std::net::TcpStream;
use futures_util::io::AsyncReadExt;

/// wraps the client
pub async fn client(port: u16) -> Result<()> {
    let config = Config::load()?;

    let stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!("connection established to: {}:{}", stream.peer_addr()?.ip(), port);
    let (_read, mut write) = stream.split();

    let join: Packet = Join::new(config.user).to_packet()?;
    join.write(&mut write).await?;

    // testing stuffs
    let message: Packet =
        Message::new("Isabelle".to_owned(), "Hello Server".to_owned()).to_packet()?;
    message.write(&mut write).await?;

    loop {}
}
