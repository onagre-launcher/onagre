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
}
