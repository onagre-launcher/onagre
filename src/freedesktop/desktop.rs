use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopEntryIni {
    #[serde(rename = "Desktop Entry")]
    pub content: DesktopEntry,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub keywords: Option<String>,
}

impl DesktopEntry {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        std::fs::read_to_string(path)
            .map(|content| serde_ini::from_str::<DesktopEntryIni>(&content))
            .map(|ini| ini.ok())
            .ok()
            .flatten()
            .map(|ini| ini.content)
    }
}
