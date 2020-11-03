#[derive(Debug, Deserialize)]
pub struct DesktopEntry {
    #[serde(rename = "Desktop Entry")]
    pub content: Option<DesktopEntryContent>,
}

#[derive(Debug, Deserialize)]
pub struct DesktopEntryContent {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Exec")]
    exec: String,
}

#[derive(Debug, Clone)]
pub struct OnagreEntry {
    pub name: String,
    pub exec: String,
}

impl From<&DesktopEntryContent> for OnagreEntry {
    fn from(desktop_entry: &DesktopEntryContent) -> Self {
        OnagreEntry {
            name: desktop_entry.name.clone(),
            exec: desktop_entry.exec.clone()
        }
    }
}