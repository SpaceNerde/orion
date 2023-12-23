mod server;
mod client;
mod commands;

use commands::*;
use clap::Parser;
use server::{server_handler, connection_handler};
use client::client_handler;

fn main() {
    let mut test_group_book = server::connection_handler::GroupBook::new();

    let mut test_group = server::connection_handler::Group::new("".to_string(), &test_group_book);

    test_group_book.add_group(&test_group);

    test_group.add_client("123".to_string());
    test_group.add_client("321".to_string());

    println!("{:?}", &test_group.get_id());

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