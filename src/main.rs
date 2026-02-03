mod spore_user;
mod spore_server;
mod sporecast;

use clap::{Parser, Subcommand};
use anyhow::Result;
use spore_user::SporeUser;
use sporecast::Sporecast;

#[derive(Parser)]
#[command(name = "spore-downloader", about = "Download Spore assets from users or sporecasts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download all assets from a user
    User {
        /// Spore username
        username: String,

        /// Output directory
        #[arg(short, long, default_value = "spore_assets/users")]
        output: String,
    },

    /// Download all assets from a sporecast
    Sporecast {
        /// Sporecast ID
        id: i64,

        /// Output directory
        #[arg(short, long, default_value = "spore_assets/sporecasts")]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::User { username, output } => {
            let user = SporeUser::new(username.clone());
            user.download_all_assets(output)?;
        }
        Commands::Sporecast { id, output } => {
            let sporecast = Sporecast::new(*id);
            sporecast.download_all_assets(output)?;
        }
    }
    Ok(())
}
