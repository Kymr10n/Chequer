use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;
use chequer_agent::{Host, Client};
use chequer_report::DiagnosticReport;

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
    let host = Host::new(listen);
    host.run().await
}

async fn run_client(connect: String) -> Result<()> {
    let client = Client::new(connect);
    let results = client.run().await?;
    
    // Generate and display report
    let report = DiagnosticReport::from_results(results);
    report.print_terminal();
    
    // Optionally save JSON
    if let Ok(json) = report.to_json() {
        std::fs::write("chequer-report.json", json)?;
        info!("Report saved to chequer-report.json");
    }
    
    Ok(())
}
