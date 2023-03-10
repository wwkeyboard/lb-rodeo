use std::net::SocketAddr;

use clap::Parser;
use warp::Filter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port this server will listen for connections on
    #[arg(short, long)]
    addr: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr: SocketAddr = args.addr.parse().unwrap();

    println!("Hello from the server - {}!", args.addr);

    let echo = warp::path!("echo").map(|| format!("pong"));

    warp::serve(echo).run(addr).await;
}
