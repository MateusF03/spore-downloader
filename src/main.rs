mod spore_user;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "spore-downloader", about = "Download Spore assets from users or sporecasts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    User {
        username: String,
        #[arg(short, long, default_value = "spore_assets/users")]
        output: String,
    },
    Sporecast {
        id: i64,
        #[arg(short, long, default_value = "spore_assets/sporecasts")]
        output: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::User { username, output } => {
            println!("Downloading assets for user: {} into {}", username, output);
        }
        Commands::Sporecast { id, output } => {
            println!("Downloading assets for sporecast ID: {} into {}", id, output);
        }
    }
    Ok(())
}
