use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct DesktopEntryIni {
    #[serde(rename = "Desktop Entry")]
    pub content: DesktopEntry,
}

#[derive(Debug, Deserialize)]
pub struct DesktopEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Exec")]
    pub exec: String,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Keywords")]
    pub keywords: Option<String>,
}

impl DesktopEntry {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        std::fs::read_to_string(path)
            .map(|content| serde_ini::from_str::<DesktopEntryIni>(&content))
            .map(|ini| ini.ok())
            .ok()
            .flatten()
            .map(|ini| ini.content)
    }
}
