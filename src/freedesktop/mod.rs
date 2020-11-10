use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;
use std::fs;

mod icons;

const BASE_DIRS: &[&str] = &["/usr/share/icons", "/usr/share/pixmaps"];

#[derive(Debug, Deserialize)]
pub struct IconTheme {
    #[serde(rename = "Icon Theme")]
    // This might not be usefull since we work with path only
    pub data: HashMap<String, String>,
    #[serde(flatten)]
    pub entries: HashMap<String, DirEntry>,
}

#[derive(Debug, Deserialize)]
pub struct DirEntry {
    #[serde(rename = "Size")]
    size: String,
}

// see : https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
pub struct IconRequest {
    pub theme_name: String,
    pub icon_name: String,
    pub size: u32,
    // lazily set from request
    theme: Option<IconTheme>,
    theme_path: Option<PathBuf>,
}


impl IconRequest {
    // Must be called after `load`
    pub fn lookup(&mut self) -> Result<Vec<PathBuf>> {
        self.load_theme()?;

        // Get entry from index.theme
        let subdirs = &self.theme.as_ref();
        let subdirs: Vec<&String> = subdirs.unwrap().entries
            .iter()
            .filter(|(_, entry)| entry.size == self.size.to_string())
            .map(|(dir, _)| dir)
            .collect();

        // Collect entries absolute paths
        Ok(self.search_icon(subdirs))
    }

    fn search_icon(&self, entries: Vec<&String>) -> Vec<PathBuf> {
        use glob::glob;
        let mut matches = vec![];
        let theme_path = &self.theme_path.as_ref().unwrap();
        for subdir in entries.iter() {
            let path = theme_path.join(subdir);
            let path = path.to_str().unwrap();
            let glob_pattern = format!("{}/**/{}.*", path, &self.icon_name);
            for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => matches.push(path),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }

        matches
    }

    fn load_theme(&mut self) -> Result<()> {
        // Fallbacks to paths defineds in the specs
        let theme_name = &self.theme_name;
        let fallback = || if Path::new(BASE_DIRS[0]).join(theme_name).exists() {
            Ok(PathBuf::from(BASE_DIRS[0]).join(theme_name))
        } else if Path::new(BASE_DIRS[1]).join(theme_name).exists() {
            Ok(PathBuf::from(BASE_DIRS[1]).join(theme_name))
        } else {
            Err(anyhow!("Unable to locate theme {}", theme_name))
        };

        // First we try user dir then fallback to the standard paths
        let path = if let Some(path) = dirs::data_dir().map(|path| path.join(theme_name)) {
            if path.exists() { Ok(path) } else { fallback() }
        } else { fallback() }?;

        // Try to deserialize theme from path
        let theme = fs::read_to_string(&path.join("index.theme"))?;
        let theme = serde_ini::from_str::<IconTheme>(&theme)?;

        self.theme = Some(theme);
        self.theme_path = Some(path);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::freedesktop::{IconRequest};

    #[test]
    fn get_adawaita_icons() {
        let mut request = IconRequest {
            theme_name: "Adwaita".to_string(),
            icon_name: "multimedia-volume-control-symbolic".to_string(),
            size: 24,
            theme: None,
            theme_path: None
        };

        assert!(!request.lookup().unwrap().is_empty());
    }
}


