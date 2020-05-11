// namespacing
use crate::packet::{Message, Packet};
use crate::Result;
use async_std::net::TcpStream;
use std::convert::TryInto;

/// wraps the client
pub async fn client(port: u16) -> Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!("connection established to: {}:{}", stream.peer_addr()?.ip(), port);

    // testing stuffs
    let message: Packet =
        Message::new("Isabelle".to_owned(), "Hello Server".to_owned()).try_into()?;
    message.write(&mut stream).await?;

    Ok(())
}
