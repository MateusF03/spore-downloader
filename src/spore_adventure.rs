use crate::{
    progress::create_progress_bar,
    spore_server::{Asset, AssetType, SporeServer},
};
use anyhow::{Context, Result, bail};

pub struct SporeAdventure {
    pub id: i64,
}

impl SporeAdventure {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
    pub fn validate_adventure(&self) -> Result<()> {
        let server = SporeServer::new();
        let xml = server
            .get_adventure_xml(self.id)
            .context("Failed to download adventure XML")?;

        if !xml.contains("Scenario") {
            bail!("ID {} is not a valid adventure", self.id);
        }
        Ok(())
    }

    pub fn get_all_assets(&self, separate_by_type: bool) -> Result<Vec<Asset>> {
        let server = SporeServer::new();
        let xml = server
            .get_adventure_xml(self.id)
            .context("Failed to download adventure XML")?;

        let assets_start = xml.find("<assets><asset>");
        if assets_start.is_none() {
            return Ok(vec![]);
        }

        let start_pos = assets_start.unwrap() + 15; // length of "<assets><asset>"
        let end_pos = xml.find("<cScenarioResource>").unwrap() - 17;

        let assets_section = &xml[start_pos..end_pos];
        let asset_ids: Vec<&str> = assets_section.split("</asset><asset>").collect();

        let mut assets = Vec::new();
        for id_str in asset_ids {
            let id: i64 = id_str
                .parse()
                .with_context(|| format!("Failed to parse asset ID: {id_str}"))?;
            let asset_type = if separate_by_type {
                server.get_asset_type(id).unwrap_or(AssetType::Unknown)
            } else {
                AssetType::Unknown
            };
            assets.push(Asset { id, asset_type });
        }

        println!(
            "Found {} required assets for adventure {}",
            assets.len(),
            self.id
        );
        Ok(assets)
    }

    pub fn download_all_assets(&self, output_dir: &str, separate_by_type: bool) -> Result<()> {
        self.validate_adventure()?;

        let server = SporeServer::new();
        let mut adventure_path = std::path::PathBuf::from(output_dir);

        std::fs::create_dir_all(&adventure_path)
            .with_context(|| format!("Failed to create directory {}", adventure_path.display()))?;

        if separate_by_type {
            adventure_path.push(AssetType::Adventure.dir_name());
            std::fs::create_dir_all(&adventure_path)
                .with_context(|| "Failed to create adventure directory".to_string())?;
        }

        adventure_path.push(format!("{}.png", self.id));
        server
            .download_asset_png(self.id, &adventure_path)
            .with_context(|| "Failed to download adventure PNG".to_string())?;
        println!(
            "Downloaded adventure {} to {}",
            self.id,
            adventure_path.display()
        );

        let assets = self.get_all_assets(separate_by_type)?;

        let pb = create_progress_bar(assets.len() as u64);
        pb.set_message("downloading assets");

        for asset in &assets {
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
            pb.println(format!(
                "  Downloaded asset ID {} to {}",
                asset.id,
                asset_path.display()
            ));
            pb.inc(1);
        }
        pb.finish_with_message(format!(
            "Downloaded {} assets for adventure {}",
            assets.len(),
            self.id
        ));
        Ok(())
    }
}
