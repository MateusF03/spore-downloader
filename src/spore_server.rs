use anyhow::Result;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[derive(Debug)]
pub struct Asset {
    pub id: i64,
    pub asset_type: AssetType,
}

#[derive(Debug)]
pub enum AssetType {
    Creature,
    Vehicle,
    Building,
    Ufo,
    Adventure,
    Unknown,
}

impl AssetType {
    pub fn dir_name(&self) -> &'static str {
        match self {
            AssetType::Creature => "creatures",
            AssetType::Vehicle => "vehicles",
            AssetType::Building => "buildings",
            AssetType::Ufo => "ufo",
            AssetType::Adventure => "adventures",
            AssetType::Unknown => "unknown",
        }
    }
}

pub struct SporeServer {
    endpoint: String,
    client: Client,
}

impl SporeServer {
    pub fn new() -> Self {
        Self {
            endpoint: "http://www.spore.com".to_string(),
            client: Client::new(),
        }
    }

    pub fn get_assets_from_user_feed(&self, username: &str) -> Result<String> {
        self.get_text(&format!("{}/atom/assets/user/{}", self.endpoint, username))
    }

    pub fn get_sporecast_feed(&self, sporecast_id: i64) -> Result<String> {
        self.get_text(&format!(
            "{}/atom/sporecast/{}",
            self.endpoint, sporecast_id
        ))
    }

    fn get_text(&self, url: &str) -> Result<String> {
        Ok(self.client.get(url).send()?.text()?)
    }

    pub fn download_asset_png(&self, asset_id: i64, file_path: &Path) -> Result<()> {
        let id = asset_id.to_string();
        if id.len() < 9 {
            anyhow::bail!("asset id {} is too short", id);
        }
        let sub1 = &id[0..3];
        let sub2 = &id[3..6];
        let sub3 = &id[6..9];
        let url = format!(
            "{}/static/thumb/{}/{}/{}/{}.png",
            self.endpoint, sub1, sub2, sub3, id
        );

        let mut response = self.client.get(&url).send()?.error_for_status()?;

        let mut file = File::create(file_path)?;
        copy(&mut response, &mut file)?;
        Ok(())
    }
}
