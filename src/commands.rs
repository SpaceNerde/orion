use std::ops::RangeInclusive;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    StartServer(StartServerCommand),
    StartClient(StartClientCommand),
}

#[derive(Args)]
pub struct StartServerCommand {
    #[arg(long, default_value = "0.0.0.0", value_name = "SERVER IP")]
    pub ip: String,
    #[arg(value_parser = port_in_range)]
    pub port: i32,
}

#[derive(Args)]
pub struct StartClientCommand {
    #[arg(long, default_value = "0.0.0.0", value_name = "TARGET SERVER IP")]
    pub ip: String,
    #[arg(value_parser = port_in_range)]
    pub port: i32,
}

const PORT_RANGE: RangeInclusive<i32> = 1..=65535;

fn port_in_range(s: &str) -> Result<i32, String> {
    let port: i32 = s
        .parse()
        .map_err(|e| format!("{s} is not a port"))?;
    match PORT_RANGE.contains(&port) {
        true => {
            Ok(port as i32)
        }
        false => {
            Err(format!(
                "port not in range {} : {}",
                PORT_RANGE.start(),
                PORT_RANGE.end()
            ))
        }
    }
}