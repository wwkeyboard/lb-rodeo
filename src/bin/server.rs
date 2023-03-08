use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// port this server will listen for connections on
    #[arg(short, long)]
    port: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello from the client - {}!", args.port)
}
