use clap::{ArgGroup, Parser, Subcommand};
use log::{debug, info, warn};

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
        id_of_account: i32,
        starting_balance: f64,
    },
    Transfer {
        from_account: i32,
        to_account: i32,
        amount: f64,
    },
    Balance,
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
        Commands::CreateAccount { id_of_account, starting_balance } => {
            info!(
                "Creating a new account with ID {} and starting balance {} on port {}...",
                id_of_account, starting_balance, cli.port
            );
            create_account(*id_of_account, *starting_balance, cli.port);
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
        Commands::Balance => {
            info!("Checking balance on port {}...", cli.port);
            check_balance(cli.port);
        }
    }
}

fn start_node(port: u16) {
    info!("Server is running on port {}. Press Ctrl-C to stop.", port);
    info!("Server stopped.");
}

fn create_account(id_of_account: i32, starting_balance: f64, port: u16) {
    info!(
        "Account with ID {} created with starting balance {} on port {}.",
        id_of_account, starting_balance, port
    );
}

fn transfer(from_account: i32, to_account: i32, amount: f64, port: u16) {
    info!(
        "Transferred {} from account {} to account {} on port {}.",
        amount, from_account, to_account, port
    );
}

fn check_balance(port: u16) {
    info!("Balance checked on port {}.", port);
}
