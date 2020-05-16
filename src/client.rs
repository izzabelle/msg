// namespacing
use crate::config::ClientConfig as Config;
use crate::Result;
use async_std::net::TcpStream;

/// wraps the client
pub async fn client(port: u16) -> Result<()> {
    let _config = Config::load()?;

    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!("connection established to: {}:{}", stream.peer_addr()?.ip(), port);

    /*let (_read, mut write) = stream.split();*/

    let message =
        ilmp::Message::new("Isabelle".to_string(), "new message protocol working".to_string());
    ilmp::write(&mut stream, message).await?;

    loop {}
}
