// Command line parsing
use crate::db_connection::DB_NAME;
use clap::{ArgAction, Parser, Subcommand};
use std::env;
use std::env::*;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IPv4 address of the SpacetimeDB server
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    /// Port number
    #[arg(short, long, default_value = "3000")]
    port: u16,

    // Clear token
    #[clap(long, short, action)]
    clear: bool,
}

pub fn parse_args() -> String {
    let args = Args::parse();
    // Validate the IP address
    //if let Err(e) = args.ip.parse::<std::net::Ipv4Addr>() {
    //    panic!("Invalid IPv4 address provided: {}", e);
    // }
    if args.clear {
        let home_dir = env::var("HOME").expect("Failed to get home directory");
        let file_path = format!("{}/.spacetimedb_client_credentials/{}", home_dir, DB_NAME);
        fs::remove_file(&file_path).expect("Failed to remove file");
    }

    println!("http://{}:{}", args.ip, args.port);
    // Construct the connection URL
    format!("http://{}:{}", args.ip, args.port)
}
