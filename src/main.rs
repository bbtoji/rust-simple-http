use crate::httpserver::HttpServer;
use clap::{Parser};

mod httpserver;
mod utils;

#[derive(Parser, Debug)]
#[command(author = "bbtoji", about = "Simple http server")]
struct Args {
    #[arg(short, long, default_value_t = 8000)]
    // Port for listening http connections
    port: u16,
}

fn main() {
    let args = Args::parse();
    let http_server = HttpServer::new("0.0.0.0", args.port);
    http_server.run();
}
