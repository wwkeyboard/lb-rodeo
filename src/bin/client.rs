use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use reqwest::Client;
use serde::Serialize;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// address of the LB to test
    #[arg(short, long)]
    target: String,

    /// delay in ms between requests
    #[arg(short, long)]
    delay: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Hello from the client - {}!", &args.target);

    loop {
        let target = args.target.clone();
        tokio::spawn(async move { make_request(target).await });
        sleep(Duration::from_millis(args.delay)).await;
    }
}

async fn make_request(target: String) -> Result<()> {
    println!("requesting - ");
    
    let payload = Request{id: 1};
    let client = reqwest::Client::new();

    let resp = client.post(target)
        .json(&payload)
        .send()
        .await?;

    println!("got back = {:?}", resp);
    Ok(())
}

#[derive(Serialize)]
struct Request {
    id: u64
}
