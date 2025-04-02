// Command line parsing
use clap::{Parser, Subcommand};
use std::env::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IPv4 address of the SpacetimeDB server
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    /// Port number
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

pub fn parse_args() -> String {
    let args = Args::parse();
    // Validate the IP address
    if let Err(e) = args.ip.parse::<std::net::Ipv4Addr>() {
        panic!("Invalid IPv4 address provided: {}", e);
    }

    println!("http://{}:{}", args.ip, args.port);
    // Construct the connection URL
    format!("http://{}:{}", args.ip, args.port)
}
