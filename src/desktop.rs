#[derive(Debug, Deserialize)]
pub struct DesktopEntry {
    #[serde(rename = "Desktop Entry")]
    pub entry: Option<DesktopEntryContent>,
}

#[derive(Debug, Deserialize)]
pub struct DesktopEntryContent {
    #[serde(rename = "Name")]
    pub name: String
}