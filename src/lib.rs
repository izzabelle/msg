use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn server(port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", &port)).await?;
    println!("online as server at: {}:{}", listener.local_addr()?.ip(), port);
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        // handle stream
        println!("new stream from: {}", stream.peer_addr()?.ip());
        stream.write_all(b"hello world").await?;
    }

    Ok(())
}

pub async fn client(port: u16) -> Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", &port)).await?;
    println!("connection established to: {}:{}", stream.peer_addr()?.ip(), port);
    let mut buf = vec![0u8; 1024];
    stream.read(&mut buf).await?;
    println!("{}", String::from_utf8_lossy(&mut buf));
    Ok(())
}
