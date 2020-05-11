// namespacing
/*use crate::packet::{Message, Packet, PacketType};*/
use crate::Result;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
/*use async_std::task;*/
use futures::io::WriteHalf;
use lazy_static::lazy_static;
use std::collections::HashMap;
/*use std::convert::TryFrom;*/
use futures_util::io::AsyncReadExt;
use std::sync::Mutex;
use uuid::Uuid;

lazy_static! {
    static ref WRITE_STREAMS: Mutex<HashMap<Uuid, WriteHalf<TcpStream>>> =
        Mutex::new(HashMap::new());
}

/// wraps the server
pub async fn server(port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", &port)).await?;
    println!("online as server at: {}:{}", listener.local_addr()?.ip(), port);
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        println!("new stream from: {}", &stream.peer_addr()?.ip());
        let (read, write) = stream.split();
        let stream_id = Uuid::new_v4();
        WRITE_STREAMS.lock()?.insert(stream_id, write);

        // handle stream
        /*        task::spawn(async {
            loop {
                let packet = Packet::read(&mut stream).await?;
                let message = match packet.packet_type {
                    PacketType::Message => Message::try_from(packet),
                };
                println!("{:?}", message);
            }
        });*/
    }

    Ok(())
}
