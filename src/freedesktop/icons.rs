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
    pub size: String,
    #[serde(rename = "Type")]
    pub scale: Scale,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Scale {
    Fixed,
    Scalable,
    Threshold,
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

        extension.map(|extention| IconPath {
            path,
            extension: extention,
        })
    }
}

#[derive(Debug)]
pub struct IconFinder {
    theme_paths: Vec<(PathBuf, IconTheme)>,
    fallbacks: Vec<(PathBuf, IconTheme)>,
}

impl IconFinder {
    // see : https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
    pub fn build(theme_name: &str) -> Result<Self> {
        let mut theme_paths = vec![];
        let mut fallbacks = vec![];

        // Theme /usr/share/icons/hicolor
        let hicolor_usr_share_path = PathBuf::from(BASE_DIRS[0]).join("hicolor");

        let hicolor_usr_share_index =
            fs::read_to_string(&hicolor_usr_share_path.join("index.theme"));
        if let Ok(index) = hicolor_usr_share_index {
            if let Ok(theme) = serde_ini::from_str::<IconTheme>(&index) {
                fallbacks.push((hicolor_usr_share_path, theme));
            }
        }

        // Named theme in /usr/share/icons
        let path = IconFinder::theme_path(theme_name);

        if let Some(theme_path) = path {
            let index_path = theme_path.join("index.theme");
            let theme = fs::read_to_string(&index_path)?;
            let theme = serde_ini::from_str::<IconTheme>(&theme)?;

            // Push all parent path to the request
            if let Some(parents) = theme.data.get("Inherits") {
                let parents = parents
                    .trim()
                    .split(',')
                    .filter_map(|parent_name| IconFinder::theme_path(parent_name))
                    .map(|path| {
                        fs::read_to_string(path.join("index.theme")).map(|content| (path, content))
                    })
                    .filter_map(Result::ok)
                    .map(|(path, content)| {
                        serde_ini::from_str::<IconTheme>(content.as_str())
                            .map(|result| (path, result))
                    })
                    .filter_map(Result::ok)
                    .collect::<Vec<(PathBuf, IconTheme)>>();

                // last we push the current theme path
                theme_paths.push((theme_path, theme));

                fallbacks.extend(parents);
            }
        }

        Ok(IconFinder {
            theme_paths,
            fallbacks,
        })
    }

    fn theme_path(theme_name: &str) -> Option<PathBuf> {
        let usr_share_theme = Path::new(BASE_DIRS[0]).join(&theme_name);
        let usr_local_theme = dirs::data_dir().map(|path| path.join(&theme_name));

        if usr_share_theme.exists() {
            Some(usr_share_theme)
        } else {
            match usr_local_theme {
                Some(path) if path.exists() => Some(path),
                _ => None,
            }
        }
    }
}

impl IconFinder {
    //TODO: this
    pub fn lookup(&self, icon_name: &str, size: u32) -> Result<IconPath> {
        // Search icon in user theme
        for (theme_path, theme) in &self.theme_paths {
            for glob in IconFinder::get_globs(size, theme_path, theme, icon_name) {
                if let Some(path) = self.search_icon(&glob) {
                    return Ok(path);
                }
            }
        }

        // No luck going to fallback/default themes
        for (theme_path, theme) in &self.fallbacks {
            for glob in IconFinder::get_globs(size, theme_path, theme, icon_name) {
                if let Some(path) = self.search_icon(&glob) {
                    return Ok(path);
                }
            }
        }

        if let Some(data_dir) = dirs::data_dir() {
            let path = data_dir.join("icons").join("hicolor");
            let path = path.to_str().unwrap();
            let glob = format!("{}/{}x{}/**/{}.*", path, size, size, icon_name);
            println!("{}", glob);
            if let Some(path) = self.search_icon(&glob) {
                return Ok(path);
            }
        }

        // This is our last chance
        let pixmap = PathBuf::from(BASE_DIRS[1]);
        let glob = format!("{}/{}.*", pixmap.to_str().unwrap(), icon_name);
        self.search_icon(&glob)
            .ok_or_else(|| anyhow!("Icon not found"))
    }

    fn get_globs(
        size: u32,
        theme_path: &PathBuf,
        theme: &IconTheme,
        icon_name: &str,
    ) -> Vec<String> {
        theme
            .entries
            .iter()
            .filter(|(_, entry)| entry.size == size.to_string() || entry.scale == Scale::Scalable)
            .map(|(dir, _)| dir)
            .map(|dir| theme_path.join(dir))
            .map(|path| format!("{}/{}.*", path.to_str().unwrap(), icon_name))
            .collect()
    }

    fn search_icon(&self, pattern: &str) -> Option<IconPath> {
        use glob::glob;

        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if let Some(icon_path) = IconPath::try_from(path) {
                        return Some(icon_path);
                    }
                }
                Err(e) => eprintln!("No match {:?}", e),
            }
        }

        None
    }
}
