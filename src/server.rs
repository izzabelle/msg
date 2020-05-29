// namespacing
use crate::Result;
use async_std::{
    net::{TcpListener, TcpStream},
    task,
};
use futures::{
    channel::mpsc,
    io::{ReadHalf, WriteHalf},
    stream::StreamExt,
};
use futures_util::{io::AsyncReadExt, sink::SinkExt};
use ilmp::{encrypt::SymmetricEncrypt, Packet};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

// server config
#[derive(Deserialize)]
struct ServerConfig {
    ip: String,
    port: u32,
}

/// run the server
pub async fn run() -> Result<()> {
    // create ip and init tcp listener
    let config: ServerConfig = toml::from_str(&std::fs::read_to_string("./server_config.toml")?)?;
    let ip = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(ip).await?;
    let mut incoming = listener.incoming();

    // set up channels and spawn relay_handler task
    let (mut sender, receiver) = mpsc::unbounded();
    let _ = task::spawn(relay_handler(receiver));

    while let Some(stream) = incoming.next().await {
        // split streams to read / write
        let stream = stream?;
        let (mut read, mut write) = stream.split();
        let stream_id = Uuid::new_v4();

        // initialize connection and gen encryption key
        let key = ilmp::initialize_connection(&mut read, &mut write).await?;
        let encryption = SymmetricEncrypt::new(key);
        // send write stream and id to relay
        sender
            .send(RelayEvent::PeerConnected {
                stream_id: stream_id.clone(),
                write,
                encryption: encryption.clone()?,
            })
            .await?;

        // spawn read_handler task
        task::spawn(read_handler(read, sender.clone(), encryption, stream_id)).await?;
    }

    Ok(())
}

// used to pass message in channel
enum RelayEvent {
    // triggered when new client connects
    PeerConnected { stream_id: Uuid, write: WriteHalf<TcpStream>, encryption: SymmetricEncrypt },
    // triggered when a client disconnects
    PeerDisconnected { stream_id: Uuid },
    // triggered when a packet needs to be relayed
    RelayPacket { packet: Packet },
}

// channel type wrappers
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

// handles relay events
async fn relay_handler(mut receiver: Receiver<RelayEvent>) -> Result<()> {
    // used to store connections
    let mut peers: HashMap<Uuid, (WriteHalf<TcpStream>, SymmetricEncrypt)> = HashMap::new();

    // loop through incoming events
    while let Some(event) = receiver.next().await {
        match event {
            RelayEvent::PeerConnected { stream_id, write, encryption } => {
                peers.insert(stream_id, (write, encryption));
            }
            RelayEvent::PeerDisconnected { stream_id } => {
                peers.remove(&stream_id);
            }
            RelayEvent::RelayPacket { packet } => todo!(),
        }
    }

    Ok(())
}

// handle read half stream
async fn read_handler(
    mut read: ReadHalf<TcpStream>,
    mut sender: Sender<RelayEvent>,
    encryption: SymmetricEncrypt,
    stream_id: Uuid,
) -> Result<()> {
    // loop through incoming packets until disconnected
    while let Some(packet) = ilmp::read(&mut read, &encryption).await? {
        sender.send(RelayEvent::RelayPacket { packet }).await?;
    }

    // send disconnect to relay handler
    sender.send(RelayEvent::PeerDisconnected { stream_id }).await?;

    Ok(())
}

/*
async fn relay_packet<T, E>(packet: T, encryption: &E) -> Result<()>
where
    T: Clone + Sendable,
    E: Encryption,
{
    let mut locked_write_streams = WRITE_STREAMS.lock().await;
    let stream = futures::stream::iter(locked_write_streams.iter_mut());

    let packet = &packet;
    stream
        .for_each_concurrent(None, |(stream_id, mut stream)| async move {
            println!("relaying packet to: {}", stream_id.as_u128());
            let packet = packet.clone();
            // in case any of the writes fail just ignore them
            let _ = ilmp::write(&mut stream, packet, encryption).await;
        })
        .await;
    Ok(())
}
*/
