// namespacing
use crate::Result;
use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use futures::io::{ReadHalf, WriteHalf};
use futures_util::{io::AsyncReadExt, stream::StreamExt};
use ilmp::Sendable;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
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

async fn handle_stream(mut stream: ReadHalf<TcpStream>, _stream_id: Uuid) -> Result<()> {
    loop {
        let packet = ilmp::read(&mut stream).await?;
        if let Some(packet) = packet {
            let res = match packet.kind {
                ilmp::PacketKind::Message => ilmp::Message::from_packet(packet),
            };
            println!("{:?}", res);
        }
    }
    /*    println!("disconnecting");

    WRITE_STREAMS.lock().expect("failed to aqcuire lock").remove(&stream_id);

    Ok(())*/
}

/*async fn relay_packet<T: Clone + Sendable>(packet: T) -> Result<()> {
    let mut locked_write_streams = WRITE_STREAMS.lock().expect("failed to aqcuire lock");
    let stream = futures::stream::iter(locked_write_streams.iter_mut());

    let packet = &packet;
    stream.for_each_concurrent(None, |(_, mut stream)| async move {
        let packet = packet
            .clone()
            .to_packet()
            .expect("failed to convert to packet");
        // in case any of the writes fail just ignore them
        let _ = packet.write(&mut stream);
    });
    Ok(())
}
*/
