mod client;
mod core;
mod server;

use clap::{Parser, Subcommand};
use client::{run_client, ClientCommands};
use log::info;
use server::start_node;

#[derive(Parser)]
#[clap(
    name = "my_app",
    version = "1.0",
    author = "Your Name",
    about = "A CLI application example"
)]
struct Cli {
    #[clap(short, long, default_value = "9999", global = true)]
    port: u16,

    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    StartNode,
    #[clap(flatten)]
    Client(ClientCommands), // Include ClientCommands as a variant
}

fn setup_logger(verbose: bool) -> Result<(), fern::InitError> {
    let log_level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            if record.level() == log::Level::Info {
                out.finish(format_args!("{}", message))
            } else {
                out.finish(format_args!("[{}] {}", record.level(), message))
            }
        })
        .level(log::LevelFilter::Info)
        .level_for("b", log_level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    setup_logger(cli.verbose).expect("Failed to initialize logger");

    match cli.command {
        Commands::StartNode => {
            info!("Starting the node server on port {}...", cli.port);
            let _ = start_node(cli.port);
        }
        Commands::Client(client_command) => {
            info!("Connecting to node on port {}...", cli.port);
            let _ = run_client(client_command, cli.port);
        }
    }
}
