use crate::freedesktop::icons::{Extension, IconFinder, IconPath};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct DesktopEntryIni {
    #[serde(rename = "Desktop Entry")]
    pub content: DesktopEntryInContent,
}

#[derive(Debug, Deserialize)]
pub struct DesktopEntryInContent {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Exec")]
    pub exec: String,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Keywords")]
    pub keywords: Option<String>,
}

impl DesktopEntryInContent {
    pub fn get_icon(&self, size: u32, icon_finder: &IconFinder) -> Result<IconPath> {
        let path = PathBuf::from(&self.icon);
        if path.is_absolute() && path.exists() {
            let extension = path.extension().unwrap().to_str().unwrap();
            let extension = match extension {
                "svg" => Some(Extension::SVG),
                "png" => Some(Extension::PNG),
                _ => None,
            };
            if let Some(extension) = extension {
                Ok(IconPath { path, extension })
            } else {
                Err(anyhow!("No icon"))
            }
        } else {
            icon_finder.lookup(&self.icon, size)
        }
    }
}
