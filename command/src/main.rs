use std::fmt::format;

use clap::{Parser, Subcommand};
use luminmq_server::server::LuminMQServer;
use prettytable::{Table, row};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(
    name = "LuminMQ",
    author = "HappyBoy",
    version = "0.1.0",
    next_line_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "enable server")]
    Start,
}

const LOGO: &str = r"
    __                    _       __  _______ 
   / /   __  ______ ___  (_)___  /  |/  / __ \
  / /   / / / / __ `__ \/ / __ \/ /|_/ / / / /
 / /___/ /_/ / / / / / / / / / / /  / / /_/ / 
/_____/\__,_/_/ /_/ /_/_/_/ /_/_/  /_/\___\_\";

#[tokio::main]
async fn main() {
    println!("{}", boot_info_string());
    // enable log
    tracing_subscriber::registry().with(fmt::layer()).init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Start => {
            let _ = LuminMQServer::start().await;
        }
    }
}

pub fn boot_info_string() -> String {
    let mut table = Table::new();
    table.add_row(row!["Name", "Version", "Address", "Port"]);
    table.add_row(row!["✨ LuminMQ", "v0.1.0", "0.0.0.0", "8080"]);
    format!("{}\n{}", LOGO, table.to_string())
}
