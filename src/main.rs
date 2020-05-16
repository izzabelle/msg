// namespacing
use structopt::StructOpt;

// cli options
#[derive(StructOpt)]
#[structopt(
    name = "msg",
    about = "I have no idea what i'm doing but this is async"
)]
struct Opt {
    /// start the application as a server
    #[structopt(short = "s", long = "server")]
    server: bool,

    /// port, defaults to 1337
    #[structopt(short = "p", long = "port")]
    port: Option<u16>,
}

#[async_std::main]
async fn main() {
    let options = Opt::from_args();
    let port = if let Some(port) = options.port {
        port
    } else {
        1337
    };
    match options.server {
        true => {
            if let Err(err) = msg::server(port).await {
                println!("an error occured: {:?}", err);
            }
        }
        false => {
            if let Err(err) = msg::client(port).await {
                println!("an error occured: {:?}", err);
            }
        }
    }
}
