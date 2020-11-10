use crate::freedesktop::icons::{Extension, IconPath, IconRequest};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct DesktopEntryIni {
    #[serde(rename = "Desktop Entry")]
    pub content: Option<DesktopEntryInContent>,
}

#[derive(Debug, Deserialize)]
pub struct DesktopEntryInContent {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Exec")]
    pub exec: String,
    #[serde(rename = "Icon")]
    pub icon: String,
}

impl DesktopEntryInContent {
    pub fn get_icon(&self, size: u32, theme: &str) -> Result<IconPath> {
        let path = PathBuf::from(&self.icon);
        if path.is_absolute() && path.exists() {
            let extention = path.extension().unwrap().to_str().unwrap();
            let extention = match extention {
                "svg" => Some(Extension::SVG),
                "png" => Some(Extension::PNG),
                _ => None,
            };
            if let Some(extention) = extention {
                Ok(IconPath { path, extension: extention })
            } else {
                Err(anyhow!("No icon"))
            }
        } else {
            let mut request = IconRequest::new(theme.to_string(), self.icon.clone(), size);
            let paths = request.lookup()?;
            paths.first().cloned().ok_or_else(|| anyhow!("No Icon "))
        }
    }
}
