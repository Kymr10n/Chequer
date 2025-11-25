use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

#[derive(Parser)]
#[command(name = "chequer")]
#[command(about = "Steam Remote Play diagnostic tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as host (gaming PC)
    Host {
        /// Address to listen on
        #[arg(short, long, default_value = "0.0.0.0:7777")]
        listen: String,
    },
    /// Run as client (Steam Deck)
    Client {
        /// Host address to connect to
        #[arg(short, long)]
        connect: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Host { listen } => {
            info!("Starting chequer in HOST mode, listening on {}", listen);
            run_host(listen).await?;
        }
        Commands::Client { connect } => {
            info!("Starting chequer in CLIENT mode, connecting to {}", connect);
            run_client(connect).await?;
        }
    }

    Ok(())
}

async fn run_host(listen: String) -> Result<()> {
    info!("Host mode not yet implemented");
    // TODO: Implement host logic
    Ok(())
}

async fn run_client(connect: String) -> Result<()> {
    info!("Client mode not yet implemented");
    // TODO: Implement client logic
    Ok(())
}
