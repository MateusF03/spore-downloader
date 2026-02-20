mod spore_adventure;
mod spore_server;
mod spore_user;
mod sporecast;

use anyhow::Result;
use clap::{Parser, Subcommand};
use spore_adventure::SporeAdventure;
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

        /// Separate assets into subdirectories by type
        #[arg(short, long, default_value_t = false)]
        separate_by_type: bool,
    },

    /// Download all assets from a sporecast
    Sporecast {
        /// Sporecast ID
        id: i64,

        /// Output directory
        #[arg(short, long, default_value = "spore_assets/sporecasts")]
        output: String,

        /// Separate assets into subdirectories by type
        #[arg(short, long, default_value_t = false)]
        separate_by_type: bool,
    },

    /// Download all assets from an adventure
    Adventure {
        /// Spore adventure ID
        id: i64,

        /// Output directory
        #[arg(short, long, default_value = "spore_assets/adventures")]
        output: String,

        /// Separate assets into subdirectories by type
        #[arg(short, long, default_value_t = false)]
        separate_by_type: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::User {
            username,
            output,
            separate_by_type,
        } => {
            let user = SporeUser::new(username.clone());
            user.download_all_assets(output, *separate_by_type)?;
        }
        Commands::Sporecast {
            id,
            output,
            separate_by_type,
        } => {
            let sporecast = Sporecast::new(*id);
            sporecast.download_all_assets(output, *separate_by_type)?;
        }
        Commands::Adventure {
            id,
            output,
            separate_by_type,
        } => {
            // @TODO: Implement adventure downloading;
            let adventure = SporeAdventure::new(*id);
            println!("Downloading assets for adventure ID {}", adventure.id);
        }
    }
    Ok(())
}
