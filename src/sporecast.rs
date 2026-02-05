use crate::spore_server::{Asset, AssetType, SporeServer};
use anyhow::{Context, Result};
use roxmltree::Document;

pub struct Sporecast {
    pub id: i64,
}

impl Sporecast {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn get_all_assets(&self) -> Result<Vec<Asset>> {
        let server = SporeServer::new();
        let mut assets = Vec::new();
        let xml = server
            .get_sporecast_feed(self.id)
            .context("Failed to download sporecast feed")?;

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

                let asset_id: i64 = entry_id
                    .split('/')
                    .nth(1)
                    .context("Unexpected <id> format")?
                    .parse()
                    .context("Failed to parse asset ID")?;

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

                assets.push(Asset {
                    id: asset_id,
                    asset_type,
                });

                println!("Found asset ID {} for sporecast {}", asset_id, self.id);
            }

            println!("Found {} assets for sporecast {}", assets.len(), self.id);
        } else {
            println!(
                "Found no assets for sporecast {}, feed did not exist",
                self.id
            );
        }

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
                    format!("Failed to create directory {:?} for asset type", asset_path)
                })?;
            }
            asset_path.push(format!("{}.png", asset.id));
            server
                .download_asset_png(asset.id, &asset_path)
                .with_context(|| format!("Failed to download asset ID {}", asset.id))?;
            println!("Downloaded asset ID {} to {:?}", asset.id, asset_path);
        }
        println!("All assets downloaded for sporecast {}", self.id);

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
