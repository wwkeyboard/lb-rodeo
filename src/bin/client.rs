use clap::Parser;

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

    let resp = reqwest::get(args.target)
        .await?
        .text()
        .await?;
    println!("got back = {:?}", resp);
    Ok(())
}
