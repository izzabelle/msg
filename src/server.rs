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
    print!("starting server... ");
    let config: ServerConfig = toml::from_str(&std::fs::read_to_string("./server_config.toml")?)?;
    let ip = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(&ip).await?;
    let mut incoming = listener.incoming();
    println!("[  Ok   ]");

    println!("server is online at: {} ", ip);

    // set up channels and spawn relay_handler task
    print!("starting relay_handler task... ");
    let (mut sender, receiver) = mpsc::unbounded();
    let _ = task::spawn(relay_handler(receiver));
    println!("[  Ok   ]");

    while let Some(stream) = incoming.next().await {
        // got new stream
        let stream = stream?;
        let stream_addr = &stream.peer_addr()?.ip();
        println!("new stream from: {}", stream_addr);

        // split streams to read / write
        let (mut read, mut write) = stream.split();
        let stream_id = Uuid::new_v4();

        // initialize connection and gen encryption key
        print!("initializing connection with {}... ", stream_addr);
        let key = ilmp::initialize_connection(&mut read, &mut write).await?;
        let encryption = SymmetricEncrypt::new(key);
        println!("[  Ok   ]");

        // send write stream and id to relay
        sender
            .send(RelayEvent::PeerConnected {
                stream_id: stream_id.clone(),
                write,
                encryption: encryption.clone()?,
            })
            .await?;

        // spawn read_handler task
        task::spawn(read_handler(read, sender.clone(), encryption, stream_id));
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
                println!("peer {} has disconnected", stream_id);
                peers.remove(&stream_id);
            }
            RelayEvent::RelayPacket { packet } => {
                for (stream_id, (write, encryption)) in peers.iter_mut() {
                    print!("relaying packet to {}... ", stream_id);
                    ilmp::write_packet(write, packet.clone(), encryption).await?;
                    println!("[  Ok   ]");
                }
            }
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
        println!("got packet from {}", stream_id);
        sender.send(RelayEvent::RelayPacket { packet }).await?;
    }

    // send disconnect to relay handler
    sender.send(RelayEvent::PeerDisconnected { stream_id }).await?;

    Ok(())
}
