use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::{
    feed_parser::parse_assets_from_feed,
    spore_server::{Asset, SporeServer},
};

pub struct SporeUser {
    pub user_name: String,
}

impl SporeUser {
    pub fn new(user_name: String) -> Self {
        Self { user_name }
    }

    pub fn get_all_assets(&self) -> Result<Vec<Asset>> {
        let server = SporeServer::new();

        let xml = server
            .get_assets_from_user_feed(&self.user_name)
            .context("Failed to download asset feed")?;

        let assets = parse_assets_from_feed(&xml)?;
        if assets.is_empty() {
            anyhow::bail!("No assets found in user feed");
        }
        println!("Found {} assets in user feed", assets.len());

        Ok(assets)
    }

    pub fn download_all_assets(&self, base_path: &str, separate_by_type: bool) -> Result<()> {
        let assets = self.get_all_assets()?;
        let server = SporeServer::new();

        let mut user_dir = PathBuf::from(base_path);
        user_dir.push(&self.user_name);
        fs::create_dir_all(&user_dir)
            .with_context(|| format!("Failed to create directory {}", user_dir.display()))?;

        for asset in assets {
            let mut asset_path = user_dir.clone();
            if separate_by_type {
                asset_path.push(asset.asset_type.dir_name());
                fs::create_dir_all(&asset_path).with_context(|| {
                    format!(
                        "Failed to create directory {} for asset type",
                        asset_path.display()
                    )
                })?;
            }
            asset_path.push(format!("{}.png", asset.id));
            server
                .download_asset_png(asset.id, &asset_path)
                .with_context(|| format!("Failed to download asset {}", asset.id))?;

            println!(
                "Downloaded asset ID {} to {}",
                asset.id,
                asset_path.display()
            );
        }
        println!(
            "Downloaded all assets for user {} into {}",
            self.user_name,
            user_dir.display()
        );
        Ok(())
    }
}
