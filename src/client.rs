// namespacing
use crate::config::ClientConfig as Config;
use crate::Result;
use async_std::net::TcpStream;
use futures_util::io::AsyncReadExt;
use ilmp::encrypt;

/// wraps the client
pub async fn client(port: u16) -> Result<()> {
    let _config = Config::load()?;

    let stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!("connection established to: {}:{}", stream.peer_addr()?.ip(), port);
    let (mut read, mut write) = stream.split();

    let key = crate::initialize_connection(&mut read, &mut write).await?;
    let encryption = encrypt::SymmetricEncrypt::new(key);
    println!("successfully hardened connection");

    let message = ilmp::Message::new(
        "Isabelle".to_owned(),
        "oh god oh fuck this shit actually works".to_owned(),
    );

    ilmp::write(&mut write, message, &encryption).await?;

    loop {}
}
