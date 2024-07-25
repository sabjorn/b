mod core;
mod commands;

use commands::{create_account, transfer, check_balance};
use core::types::AccountId;
use clap::{Parser, Subcommand};
use log::info;


#[derive(Parser)]
#[clap(name = "my_app", version = "1.0", author = "Your Name", about = "A CLI application example")]
struct Cli {
    #[clap(short, long, default_value = "9999", global = true)]
    port: u16,

    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    StartNode,
    CreateAccount {
        account: AccountId,
        starting_balance: f64,
    },
    Transfer {
        from_account: AccountId,
        to_account: AccountId,
        amount: f64,
    },
    Balance {
        account: AccountId,
    }, 
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

    match &cli.command {
        Commands::StartNode => {
            info!("Starting the node server on port {}...", cli.port);
            start_node(cli.port);
        }
        Commands::CreateAccount { account, starting_balance } => {
            info!(
                "Creating a new account with ID {} and starting balance {} on port {}...",
                account, starting_balance, cli.port
            );
            create_account(*account, *starting_balance, cli.port);
        }
        Commands::Transfer {
            from_account,
            to_account,
            amount,
        } => {
            info!(
                "Transferring {} from account {} to account {} on port {}...",
                amount, from_account, to_account, cli.port
            );
            transfer(*from_account, *to_account, *amount, cli.port);
        }
        Commands::Balance {account} => {
            info!("Checking balance on port {}...", cli.port);
            check_balance(cli.port);
        }
    }
}

fn start_node(port: u16) {
    info!("Server is running on port {}. Press Ctrl-C to stop.", port);
    info!("Server stopped.");
}


