// modules
//mod client;
mod client;
mod server;

// namespacing
use async_std::task;
use structopt::StructOpt;
use thiserror::Error;

type Result<T> = anyhow::Result<T, MsgError>;
#[derive(Debug, Error)]
pub enum MsgError {
    #[error("std::io error")]
    StdIo(#[from] std::io::Error),
    #[error("toml error")]
    Toml(#[from] toml::de::Error),
    #[error("channel send error")]
    ChannelSendError(#[from] futures_channel::mpsc::SendError),
    #[error("ilmp error")]
    Ilmp(#[from] ilmp::IlmpError),
}

// cli opts
#[derive(StructOpt)]
#[structopt(
    name = "i still need to figure that bit out yet",
    about = "I have no idea what I'm doing"
)]
struct Opt {
    /// start the application as a server
    #[structopt(short = "s", long = "server")]
    server: bool,
}

#[async_std::main]
async fn main() {
    let opts = Opt::from_args();

    match opts.server {
        true => {
            if let Err(err) = task::spawn(server::run()).await {
                println!("error occured: {:?}", err);
            }
        }
        false => {
            if let Err(err) = task::spawn(client::client()).await {
                println!("error occured: {:?}", err);
            }
        }
    }

    loop {
        task::yield_now().await;
    }
}
