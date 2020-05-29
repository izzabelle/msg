// namespacing
use serde::Deserialize;

// client config
#[derive(Deserialize)]
struct ClientConfig {
    ip: String,
    port: u32,
    user: String,
}

/*// namespacing
use crate::config::ClientConfig as Config;
use crate::Result;
use async_std::{io, net::TcpStream, task};
use futures::io::{ReadHalf, WriteHalf};
use futures_util::io::AsyncReadExt;
use ilmp::{encrypt::SymmetricEncrypt, Sendable};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config = Config::load().expect("failed to load config");
}

/// wraps the client
pub async fn client(port: u16) -> Result<()> {
    let stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!(
        "connection established to: {}:{}",
        stream.peer_addr()?.ip(),
        port
    );
    let (mut read, mut write) = stream.split();

    let key = ilmp::initialize_connection(&mut read, &mut write).await?;
    let encryption = SymmetricEncrypt::new(key);
    println!("successfully hardened connection");

    task::spawn(outgoing(write, encryption.clone()?));
    task::spawn(incoming(read, encryption));
    Ok(())
}

pub async fn outgoing(mut write: WriteHalf<TcpStream>, encryption: SymmetricEncrypt) -> Result<()> {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).await?;
        let message = ilmp::Message::new(CONFIG.user.clone(), line);
        ilmp::write(&mut write, message, &encryption).await?;
    }
}

pub async fn incoming(mut read: ReadHalf<TcpStream>, encryption: SymmetricEncrypt) -> Result<()> {
    loop {
        let packet = ilmp::read(&mut read, &encryption).await?;
        if let Some(packet) = packet {
            let res = match packet.kind {
                ilmp::PacketKind::Message => ilmp::Message::from_packet(packet),
                _ => panic!("bad packet"),
            };
            println!("{:?}", res);
        } else {
            break;
        }
    }

    Ok(())
}
*/
