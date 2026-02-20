use anyhow::{Context, Result};
use roxmltree::Document;

use crate::spore_server::{Asset, AssetType};

pub fn parse_assets_from_feed(xml: &str) -> Result<Vec<Asset>> {
    let doc = Document::parse(xml).context("Failed to parse Atom XML")?;
    let atom_ns = "http://www.w3.org/2005/Atom";

    let feed = doc
        .descendants()
        .find(|n| n.has_tag_name((atom_ns, "feed")));

    let mut assets = Vec::new();

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

            let asset_type = entry
                .children()
                .find_map(|n| {
                    if n.has_tag_name((atom_ns, "link"))
                        && n.attribute("rel").is_some_and(|r| r == "enclosure")
                    {
                        n.attribute("type")
                            .filter(|t| t.starts_with("application/x-"))
                    } else {
                        None
                    }
                })
                .map_or(AssetType::Unknown, Into::into);

            assets.push(Asset {
                id: asset_id,
                asset_type,
            });
        }
    }

    Ok(assets)
}
