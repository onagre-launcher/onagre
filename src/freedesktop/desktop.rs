use serde::Deserialize;
use std::borrow::Cow;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopEntryIni<'a> {
    #[serde(rename = "Desktop Entry")]
    pub content: DesktopEntry<'a>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesktopEntry<'a> {
    pub name: Cow<'a, str>,
    pub exec: Cow<'a, str>,
    pub icon: Option<Cow<'a, str>>,
    pub actions: Option<Cow<'a, str>>,
    pub comment: Option<Cow<'a, str>>,
    pub keywords: Option<Cow<'a, str>>,
}

impl DesktopEntry<'_> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        std::fs::read_to_string(path)
            .map(|content| serde_ini::from_str::<DesktopEntryIni>(&content))
            .map(|ini| ini.ok())
            .ok()
            .flatten()
            .map(|ini| ini.content)
    }
}