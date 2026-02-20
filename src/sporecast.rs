use crate::{
    feed_parser::parse_assets_from_feed,
    spore_server::{Asset, SporeServer},
};
use anyhow::{Context, Result};

pub struct Sporecast {
    pub id: i64,
}

impl Sporecast {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn get_all_assets(&self) -> Result<Vec<Asset>> {
        let server = SporeServer::new();
        let xml = server
            .get_sporecast_feed(self.id)
            .context("Failed to download sporecast feed")?;

        let assets = parse_assets_from_feed(&xml)?;

        Ok(assets)
    }

    pub fn download_all_assets(&self, output_dir: &str, separate_by_type: bool) -> Result<()> {
        let server = SporeServer::new();
        let assets = self.get_all_assets()?;

        for asset in assets {
            let mut asset_path = std::path::PathBuf::from(output_dir);
            if separate_by_type {
                asset_path.push(asset.asset_type.dir_name());
                std::fs::create_dir_all(&asset_path).with_context(|| {
                    format!(
                        "Failed to create directory {} for asset type",
                        asset_path.display()
                    )
                })?;
            }
            asset_path.push(format!("{}.png", asset.id));
            server
                .download_asset_png(asset.id, &asset_path)
                .with_context(|| format!("Failed to download asset ID {}", asset.id))?;
            println!(
                "Downloaded asset ID {} to {}",
                asset.id,
                asset_path.display()
            );
        }
        println!("All assets downloaded for sporecast {}", self.id);

        Ok(())
    }
}
