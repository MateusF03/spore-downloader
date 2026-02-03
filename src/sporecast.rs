use anyhow::{Context, Result};
use roxmltree::Document;
use crate::spore_server::SporeServer;

pub struct Sporecast {
    pub id: i64,
}

impl Sporecast {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn get_all_asset_ids(&self) -> Result<Vec<i64>> {
        let server = SporeServer::new();
        let mut asset_ids = Vec::new();
        let xml = server.get_sporecast_feed(self.id)
            .context("Failed to download sporecast feed")?;

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
                    "Found asset ID {} for sporecast {}",
                    asset_id, self.id
                );
            }

            println!(
                "Found {} assets for sporecast {}",
                asset_ids.len(),
                self.id
            );
        } else {
            println!(
                "Found no assets for sporecast {}, feed did not exist",
                self.id
            );
        }

        Ok(asset_ids)
    }

    pub fn download_all_assets(&self, output_dir: &str) -> Result<()> {
        let server = SporeServer::new();
        let asset_ids = self.get_all_asset_ids()?;

        for asset_id in asset_ids {
            let file_path = format!("{}/{}.png", output_dir, asset_id);
            server.download_asset_png(asset_id, std::path::Path::new(&file_path))
                .with_context(|| format!("Failed to download asset ID {}", asset_id))?;
            println!("Downloaded asset ID {} to {}", asset_id, file_path);
        }
        println!("All assets downloaded for sporecast {}", self.id);

        Ok(())
    }
}