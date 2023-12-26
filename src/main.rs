mod server;
mod client;
mod commands;

use commands::*;
use clap::Parser;
use server::{server_handler, connection_handler};
use client::client_handler;

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::StartServer(args) => {
            server_handler::start_server(format!("{}:{}", args.ip, args.port));
        },
        Commands::StartClient(args) => {
            client_handler::start_client(format!("{}:{}", args.ip, args.port));
        }
    }
}