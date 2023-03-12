use clap::Parser;
use std::collections::HashMap;
use anyhow::Result;

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

    println!("Hello from the client - {}!", args.target);

    tokio::spawn(async move { 
        make_request(args.target.clone()).await 
    });

    //println!("got back = {:?}", resp);
    Ok(())
}

async fn make_request(target: String) -> Result<()> {
    let resp = reqwest::get(target)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(())
}
