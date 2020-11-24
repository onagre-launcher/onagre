use anyhow::Result;
use glob::glob;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const BASE_DIRS: &[&str] = &["/usr/share/icons", "/usr/share/pixmaps"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconPath {
    pub path: PathBuf,
    pub extension: Extension,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub(crate) fn try_from(path: PathBuf) -> Option<IconPath> {
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

        extension.map(|extension| IconPath { path, extension })
    }

    pub fn as_str(&self) -> &str {
        self.path.to_str().unwrap()
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
        debug!("building icon finder for theme {:?}", theme_name);
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
                debug!(
                    "Initializing icon finder with icon theme path {:?}",
                    theme_path
                );
                theme_paths.push((theme_path, theme));

                debug!(
                    "Initializing icon finder with parent/fallback icon paths {:?}",
                    parents
                        .iter()
                        .map(|parent| parent.0.clone())
                        .collect::<Vec<PathBuf>>()
                );
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
    pub fn lookup(&self, icon_name: &str, size: u32) -> Option<IconPath> {
        // Search icon in user theme
        for (theme_path, theme) in &self.theme_paths {
            for glob in IconFinder::get_globs(size, theme_path, theme, icon_name) {
                if let Some(path) = self.search_icon(&glob) {
                    return Some(path);
                }
            }
        }

        // No luck, we fallback to parent themes
        for (theme_path, theme) in &self.fallbacks {
            for glob in IconFinder::get_globs(size, theme_path, theme, icon_name) {
                if let Some(path) = self.search_icon(&glob) {
                    return Some(path);
                }
            }
        }

        // Still no luck, we now look in the default "hicolor" theme
        if let Some(data_dir) = dirs::data_dir() {
            let path = data_dir.join("icons").join("hicolor");
            let path = path.to_str().unwrap();
            let glob = format!("{}/{}x{}/**/{}.*", path, size, size, icon_name);
            if let Some(path) = self.search_icon(&glob) {
                debug!("Found icon for {} in {:?}", icon_name, path);
                return Some(path);
            }
        }

        // This is our last chance
        let pixmap = PathBuf::from(BASE_DIRS[1]);
        let glob = format!("{}/{}.*", pixmap.to_str().unwrap(), icon_name);
        self.search_icon(&glob)
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
            // look for files indirection with a size matching our current target or marked as 'Scalable'
            .filter(|(_, entry)| entry.size == size.to_string() || entry.scale == Scale::Scalable)
            // Filter out the metadata, we only need the path to the icon files
            .map(|(dir, _)| dir)
            // construct the absolute path to the icon subdir
            .map(|dir| theme_path.join(dir))
            // this is our final glob
            .map(|path| format!("{}/{}.*", path.to_str().unwrap(), icon_name))
            .collect()
    }

    fn search_icon(&self, pattern: &str) -> Option<IconPath> {
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
