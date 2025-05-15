// Command line parsing
use crate::db_connection::DB_NAME;
use clap::Parser;
use dirs::*;
use once_cell::sync::Lazy;
use std::env;
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
        let token_path = Lazy::new(|| {
            dirs::home_dir()
                .unwrap()
                .join(".spacetimedb_client_credentials")
                .join(DB_NAME)
        });
        if token_path.exists() {
            fs::remove_file(&*token_path).expect("Failed to remove authentication token file");
        } else {
            eprintln!(
                "Authentication token does not exist at {}\nIgnoring '--clear' argument",
                token_path.display().to_string()
            );
        }
    }

    println!("http://{}:{}", args.ip, args.port);
    // Construct the connection URL
    format!("http://{}:{}", args.ip, args.port)
}
