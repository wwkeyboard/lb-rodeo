use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// address of the LB to test
    #[arg(short, long)]
    target: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Hello from the client - {}!", &args.target);

    loop {
        let target = args.target.clone();
        tokio::spawn(async move {
            make_request(target).await
        });
        sleep(Duration::from_millis(500)).await;
    }
}

async fn make_request(target: String) -> Result<()> {
    println!("requesting - ");
    let resp = reqwest::get(target)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("got back = {:?}", resp);
    Ok(())
}
