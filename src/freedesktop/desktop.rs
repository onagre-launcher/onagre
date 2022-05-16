use std::borrow::Cow;
use serde::Deserialize;
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
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub actions: Option<String>,
    pub comment: Option<Cow<'a,str>>,
    pub keywords: Option<String>,
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

#[cfg(test)]
mod test {
    use crate::freedesktop::desktop::DesktopEntryIni;

    #[test]
    fn test_de_deserialization() {
        let conduktor = r#"
            [Desktop Entry]
            Name=Conduktor
            Comment=Kafka Desktop Client
            Exec=conduktor
            Terminal=false
            Type=Application
            Categories=Development;
        "#;

        let conduktor: serde_ini::de::Result<DesktopEntryIni> = serde_ini::from_str(conduktor);

        assert!(conduktor.is_ok())
    }
}
