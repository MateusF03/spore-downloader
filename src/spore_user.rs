use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use roxmltree::Document;


use crate::spore_server::SporeServer;

pub struct SporeUser {
    pub user_name: String,
}

impl SporeUser {
    pub fn new(user_name: String) -> Self {
        Self { user_name }
    }

    pub fn get_all_asset_ids(&self) -> Result<Vec<i64>> {
        let server = SporeServer::new();
        let mut asset_ids = Vec::new();

        let xml = server
            .get_assets_from_user_feed(&self.user_name)
            .context("Failed to download asset feed")?;

        let doc = Document::parse(&xml)
            .context("Failed to parse Atom XML")?;

        let atom_ns = "http://www.w3.org/2005/Atom";

        let feed = doc
            .descendants()
            .find(|n| n.has_tag_name((atom_ns, "feed")));

        if let Some(feed) = feed {
            for entry in feed
                .children()
                .filter(|n| n.has_tag_name((atom_ns, "entry")))
            {
                let id_node = entry
                    .children()
                    .find(|n| n.has_tag_name((atom_ns, "id")))
                    .context("Entry missing <id> element")?;

                let entry_id = id_node
                    .text()
                    .context("<id> element was empty")?;

                let asset_id: i64 = entry_id
                    .split('/')
                    .nth(1)
                    .context("Unexpected <id> format")?
                    .parse()
                    .context("Failed to parse asset ID")?;

                asset_ids.push(asset_id);

                println!(
                    "Found asset ID {} for user {}",
                    asset_id, self.user_name
                );
            }

            println!(
                "Found {} assets for user {}",
                asset_ids.len(),
                self.user_name
            );
        } else {
            println!(
                "Found no assets for user {}, feed did not exist",
                self.user_name
            );
        }

        Ok(asset_ids)
    }

    pub fn download_all_assets(&self, base_path: &str) -> Result<()> {
        let asset_ids = self.get_all_asset_ids()?;
        let server = SporeServer::new();

        let mut user_dir = PathBuf::from(base_path);
        user_dir.push(&self.user_name);
        fs::create_dir_all(&user_dir)
            .with_context(|| format!("Failed to create directory {:?}", user_dir))?;

        for asset_id in asset_ids {
            let mut asset_path = user_dir.clone();
            asset_path.push(format!("{}.png", asset_id));
            server.download_asset_png(asset_id, &asset_path)
                .with_context(|| format!("Failed to download asset {}", asset_id))?;

            println!(
                "Downloaded asset ID {} to {:?}",
                asset_id, asset_path
            );
        }
        println!(
            "Downloaded all assets for user {} into {:?}",
            self.user_name, user_dir
        );
        Ok(())
    }
}