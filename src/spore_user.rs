use anyhow::{Context, Result};
use roxmltree::Document;
use std::fs;
use std::path::PathBuf;

use crate::spore_server::{Asset, AssetType, SporeServer};

pub struct SporeUser {
    pub user_name: String,
}

impl SporeUser {
    pub fn new(user_name: String) -> Self {
        Self { user_name }
    }

    pub fn get_all_assets(&self) -> Result<Vec<Asset>> {
        let server = SporeServer::new();
        let mut assets = Vec::new();

        let xml = server
            .get_assets_from_user_feed(&self.user_name)
            .context("Failed to download asset feed")?;

        let doc = Document::parse(&xml).context("Failed to parse Atom XML")?;

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

                let entry_id = id_node.text().context("<id> element was empty")?;

                let enclosure = entry
                    .children()
                    .filter(|n| {
                        n.has_tag_name((atom_ns, "link")) && n.attribute("rel") == Some("enclosure")
                    })
                    .find(|n| {
                        n.attribute("type")
                            .map(|t| t.starts_with("application/x-"))
                            .unwrap_or(false)
                    });

                let asset_type = enclosure
                    .and_then(|n| n.attribute("type"))
                    .map(Self::asset_type_from_mime)
                    .unwrap_or(AssetType::Unknown);

                println!(
                    "Determined asset type {:?} for entry ID {}",
                    asset_type, entry_id
                );

                let asset_id: i64 = entry_id
                    .split('/')
                    .nth(1)
                    .context("Unexpected <id> format")?
                    .parse()
                    .context("Failed to parse asset ID")?;

                assets.push(Asset {
                    id: asset_id,
                    asset_type,
                });

                println!("Found asset ID {} for user {}", asset_id, self.user_name);
            }

            println!("Found {} assets for user {}", assets.len(), self.user_name);
        } else {
            println!(
                "Found no assets for user {}, feed did not exist",
                self.user_name
            );
        }

        Ok(assets)
    }

    pub fn download_all_assets(&self, base_path: &str, separate_by_type: bool) -> Result<()> {
        let assets = self.get_all_assets()?;
        let server = SporeServer::new();

        let mut user_dir = PathBuf::from(base_path);
        user_dir.push(&self.user_name);
        fs::create_dir_all(&user_dir)
            .with_context(|| format!("Failed to create directory {:?}", user_dir))?;

        for asset in assets {
            let mut asset_path = user_dir.clone();
            if separate_by_type {
                asset_path.push(asset.asset_type.dir_name());
                fs::create_dir_all(&asset_path).with_context(|| {
                    format!("Failed to create directory {:?} for asset type", asset_path)
                })?;
            }
            asset_path.push(format!("{}.png", asset.id));
            server
                .download_asset_png(asset.id, &asset_path)
                .with_context(|| format!("Failed to download asset {}", asset.id))?;

            println!("Downloaded asset ID {} to {:?}", asset.id, asset_path);
        }
        println!(
            "Downloaded all assets for user {} into {:?}",
            self.user_name, user_dir
        );
        Ok(())
    }

    fn asset_type_from_mime(mime: &str) -> AssetType {
        match mime {
            "application/x-creature+xml" => AssetType::Creature,
            "application/x-vehicle+xml" => AssetType::Vehicle,
            "application/x-building+xml" => AssetType::Building,
            "application/x-ufo+xml" => AssetType::Ufo,
            "application/x-adventure+xml" => AssetType::Adventure,
            _ => AssetType::Unknown,
        }
    }
}
