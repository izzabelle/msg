// namespacing
use crate::config::ClientConfig as Config;
use crate::Result;
use async_std::net::TcpStream;
use futures::io::ReadHalf;
use futures_util::io::AsyncReadExt;

/// wraps the client
pub async fn client(port: u16) -> Result<()> {
    let _config = Config::load()?;

    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!(
        "connection established to: {}:{}",
        stream.peer_addr()?.ip(),
        port
    );

    let message = ilmp::Message::new(
        "Isabelle".to_owned(),
        "oh god oh fuck this shit actually works".to_owned(),
    );
    ilmp::write(&mut stream, message);

    let (read, mut write) = stream.split();

    Ok(())
}
