use std::net::SocketAddr;

use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use clap::Parser;
use serde::Serialize;

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

    let app = Router::new()
        .route("/", get(echo))
        .route("/echo", post(echo));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn echo(payload: String) -> (StatusCode, Json<Echo>) {
    println!(" - {:?}", &payload);
    let echo = Echo {
        source: 1,
        payload,
    };
    (StatusCode::CREATED, Json(echo))
}

#[derive(Serialize)]
struct Echo {
    source: usize,
    payload: String,
}
