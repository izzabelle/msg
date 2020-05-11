// namespacing
use crate::{
    packet::{Join, Message, Packet, PacketType},
    Result,
};
use async_std::{
    net::{TcpListener, TcpStream},
    prelude::*,
    task,
};
use futures::io::{ReadHalf, WriteHalf};
use futures_util::io::AsyncReadExt;
use lazy_static::lazy_static;
use std::{collections::HashMap, convert::TryFrom, sync::Mutex};
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
        let stream = stream?;
        let stream_addr = &stream.peer_addr()?.ip();

        println!("new stream from: {}", &stream_addr);

        let (read, write) = stream.split();
        let stream_id = Uuid::new_v4();

        WRITE_STREAMS.lock().expect("could not aqcuire lock").insert(stream_id.clone(), write);
        task::spawn(handle_stream(read, stream_id));
    }

    Ok(())
}

async fn handle_stream(mut stream: ReadHalf<TcpStream>, stream_id: Uuid) -> Result<()> {
    loop {
        let packet = match Packet::read(&mut stream).await {
            Ok(packet) => packet,
            Err(err) => {
                println!("error reading packet: {:?}", err);
                return Ok(());
            }
        };

        let packet = if let Some(packet) = packet {
            println!("got packet");
            packet
        } else {
            break;
        };

        match packet.packet_type {
            PacketType::Message => {
                let msg = Message::try_from(packet)?;
                println!("{:?}", msg);
            }
            PacketType::Join => {
                let join = Join::try_from(packet)?;
                println!("{:?}", join);
            }
        }

        println!("packet processed");
    }
    println!("disconnecting");

    WRITE_STREAMS.lock().expect("failed to aqcuire lock").remove(&stream_id);

    Ok(())
}

/*
async fn relay_packet() -> Result<()> {
    let locked_stream = WRITE_STREAMS.lock().

}*/
