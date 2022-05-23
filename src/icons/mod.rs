use std::collections::HashMap;
use iced_native::svg::Handle;
use iced_native::widget::Svg;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::THEME;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use pop_launcher_toolkit::launcher::IconSource;
use serde::{Deserialize, Serialize};
use crate::config::color::OnagreColor;

// We use this only for symbolic svg icons which needs to be loaded with a color theme
// For other icons, the freedesktop-icon crate has a cache already
pub(crate) static SYMBOLIC_ICON_CACHE: Lazy<Mutex<HashMap<String, Vec<u8>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// This work is licenced under the terms of either the GNU LGPL v3 or
// Creative Commons Attribution-Share Alike 3.0 United States License.
//
// To view a copy of the CC-BY-SA licence, visit
// http://creativecommons.org/licenses/by-sa/3.0/ or send a letter to Creative
// Commons, 171 Second Street, Suite 300, San Francisco, California 94105, USA.
//
// When attributing the artwork, using "GNOME Project" is enough.
// Please link to http://www.gnome.org where available.
const FALLBACK_ICON: &str = include_str!("dialog-question-symbolic.svg");
const FALLBACK_ICON_PATH: &str = "dialog-question-symbolic.svg";

// Build the fallback icon once for .row-selected and .row foreground colors
// and cache the result.
pub fn fallback_icon(color: &OnagreColor) -> Svg {
    let hex_color = color.to_string();
    let mut cache = SYMBOLIC_ICON_CACHE.lock().unwrap();
    let path = Path::new(FALLBACK_ICON_PATH);
    let key = format!("{hex_color}{path:?}");

    let svg = match cache.get(&key) {
        Some(svg) => svg,
        None => {
            let svg = inject_color_into_svg(FALLBACK_ICON, &hex_color);
            cache.insert(key.clone(), svg.into_bytes());
            cache.get(&key).unwrap()
        }
    };

    let handle = Handle::from_memory(svg.as_slice());

    Svg::new(handle)
}

fn inject_color_into_svg(content: &str, hex_color: &str) -> String {
    // Svg does not support transparency for hex colors
    let hex_color = &hex_color[0..7];

    let mut string = content.to_string();
    for (pos, _) in content.match_indices("#") {
        string.replace_range(pos..pos + 7, hex_color);
    }

    string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconPath {
    pub path: PathBuf,
    pub extension: Extension,
    pub symbolic: bool,
}

impl IconPath {
    pub fn lookup(name: &str, theme: &str, size: u16) -> Option<Self> {
        if name.ends_with("-symbolic") {
            freedesktop_icons::lookup(name)
                .with_theme(theme)
                .with_cache()
                .force_svg()
                .find()
                .and_then(|icon| Self::from_path(icon, true))
        } else {
            freedesktop_icons::lookup(name)
                .with_scale(1)
                .with_theme(theme)
                .with_size(size)
                .find()
                .and_then(|icon| Self::from_path(icon, false))
        }
    }

    pub fn absolute_from_icon_source(source: Option<&IconSource>) -> Option<Self> {
        source.and_then(|icon| {
            let (path, symbolic) = match icon {
                IconSource::Name(icon) => (icon, icon.ends_with("-symbolic")),
                IconSource::Mime(icon) => (icon, false),
            };

            IconPath::from_path(path.as_ref(), symbolic)
        })
    }

    pub fn from_source(source: &IconSource, theme: &String) -> Option<Self> {
        match source {
            IconSource::Name(name) => IconPath::lookup(name, theme, THEME.icon_size),
            IconSource::Mime(mime) => {
                let name = mime.replace('/', "-");
                IconPath::lookup(&name, theme, THEME.icon_size)
            }
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P, symbolic: bool) -> Option<Self> {
        path.as_ref()
            .extension()
            .map(|ext| ext.to_string_lossy())
            .and_then(|ext| Extension::try_from(ext.as_ref()).ok())
            .map(|extension| Self {
                path: path.as_ref().to_path_buf(),
                extension,
                symbolic,
            })
    }

    // If we have a symbolic icon try to replace the foreground color with the current
    // one and cache the result, otherwise build the svg from icon path
    pub fn  to_svg(&self, color: &OnagreColor) -> Svg {
        if self.symbolic {
            let mut icon_cache = SYMBOLIC_ICON_CACHE.lock().unwrap();
            let hex_color = color.to_string();
            let key = format!("{hex_color}{:?}", self.path);
            let svg = match icon_cache.get(&key) {
                None => {
                    let content = std::fs::read_to_string(&self.path)
                        .expect("Icon path does not exists");
                    let svg = inject_color_into_svg(&content, &hex_color);
                    icon_cache.insert(key.clone(), svg.as_bytes().to_vec());
                    icon_cache.get(&key).unwrap()
                }
                Some(svg) => svg
            };

            let handle = Handle::from_memory(svg.as_slice());
            Svg::new(handle)
        } else {
            Svg::from_path(&self.path)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Extension {
    Svg,
    Png,
}

impl TryFrom<&str> for Extension {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "png" => Ok(Extension::Png),
            "svg" => Ok(Extension::Svg),
            ext => Err(anyhow!("Unsupported icon extension: {ext}")),
        }
    }
}
