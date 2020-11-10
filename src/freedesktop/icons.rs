use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const BASE_DIRS: &[&str] = &["/usr/share/icons", "/usr/share/pixmaps"];

#[derive(Debug, Clone)]
pub struct IconPath {
    pub path: PathBuf,
    pub extension: Extension,
}

#[derive(Debug, Clone)]
pub enum Extension {
    SVG,
    PNG,
}

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

impl IconPath {
    fn try_from(path: PathBuf) -> Option<IconPath> {
        let extension = if let Some(ext) = path.extension() {
            let extension = ext.to_str().unwrap();
            match extension {
                "svg" => Some(Extension::SVG),
                "png" => Some(Extension::PNG),
                _ => None,
            }
        } else {
            None
        };

        extension.map(|extention| IconPath { path, extension: extention })
    }
}

// see : https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
pub struct IconRequest {
    pub theme_name: String,
    pub icon_name: String,
    pub size: u32,
    // lazily set from request
    theme: Option<IconTheme>,
    theme_path: Vec<PathBuf>,
}

impl IconRequest {
    pub fn new(theme_name: String, icon_name: String, size: u32) -> Self {
        let mut default_paths = vec![];

        if let Some(data_dir) = dirs::data_dir() {
            default_paths.push(data_dir.join("icons").join("hicolor"));
        };

        let hicolor_usr_share = PathBuf::from(BASE_DIRS[0]).join("hicolor");

        if hicolor_usr_share.exists() {
            default_paths.push(hicolor_usr_share);
        };

        IconRequest {
            theme_name,
            icon_name,
            size,
            theme: None,
            theme_path: default_paths,
        }
    }
}

impl IconRequest {
    // Must be called after `load`
    pub fn lookup(&mut self) -> Result<Vec<IconPath>> {
        self.load_theme()?;

        // Get entry from index.theme
        let subdirs = &self.theme.as_ref();

        let mut paths = vec![];
        // Collect entries absolute paths
        for path in self.theme_path.iter().rev() {
            let subdirs: Vec<PathBuf> = subdirs
                .unwrap()
                .entries
                .iter()
                .filter(|(_, entry)| entry.size == self.size.to_string())
                .map(|(dir, _)| path.join(dir))
                .collect();
            paths.extend(self.search_icon(subdirs));
        }

        let pixmap = PathBuf::from(BASE_DIRS[1]);

        if pixmap.exists() {
            paths.extend(self.search_icon(vec![pixmap]));
        };

        Ok(paths)
    }

    fn search_icon(&self, entries: Vec<PathBuf>) -> Vec<IconPath> {
        use glob::glob;
        let mut matches = vec![];
        for subdir in entries.iter() {
            let path = subdir.to_str().unwrap();
            let glob_pattern = format!("{}/{}.*", path, &self.icon_name);
            for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if let Some(icon_path) = IconPath::try_from(path) {
                            matches.push(icon_path);
                        }
                    }
                    Err(e) => eprintln!("No match {:?}", e),
                }
            }
        }

        matches
    }

    fn load_theme(&mut self) -> Result<()> {
        // Fallbacks to paths defineds in the specs
        let theme_name = &self.theme_name;
        let fallback = || {
            if Path::new(BASE_DIRS[0]).join(theme_name).exists() {
                Ok(PathBuf::from(BASE_DIRS[0]).join(theme_name))
            } else if Path::new(BASE_DIRS[1]).join(theme_name).exists() {
                Ok(PathBuf::from(BASE_DIRS[1]).join(theme_name))
            } else {
                match dirs::data_dir() {
                    Some(data_dir) if data_dir.join("icons").join("hicolor").exists() => {
                        Ok(data_dir.join("icons").join("hicolor"))
                    }
                    _ => Err(anyhow!("Theme not found")),
                }
            }
        };

        // First we try user dir then fallback to the standard paths
        let path = if let Some(path) = dirs::data_dir().map(|path| path.join(theme_name)) {
            if path.exists() {
                Ok(path)
            } else {
                fallback()
            }
        } else {
            fallback()
        }?;

        // Try to deserialize theme from path
        let theme = fs::read_to_string(&path.join("index.theme"))?;
        let theme = serde_ini::from_str::<IconTheme>(&theme)?;
        self.theme = Some(theme);
        self.theme_path.push(path);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::freedesktop::icons::IconRequest;
    use crate::freedesktop::IconRequest;

    #[test]
    fn get_adawaita_icons() {
        let mut request = IconRequest {
            theme_name: "Adwaita".to_string(),
            icon_name: "multimedia-volume-control-symbolic".to_string(),
            size: 24,
            theme: None,
            theme_path: None,
        };

        assert!(!request.lookup().unwrap().is_empty());
    }
}
