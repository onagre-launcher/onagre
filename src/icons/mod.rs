use iced_native::svg::Handle;
use iced_native::widget::Svg;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use crate::THEME;
use anyhow::anyhow;
use pop_launcher_toolkit::launcher::IconSource;
use serde::{Deserialize, Serialize};

// This work is licenced under the terms of either the GNU LGPL v3 or
// Creative Commons Attribution-Share Alike 3.0 United States License.
//
// To view a copy of the CC-BY-SA licence, visit
// http://creativecommons.org/licenses/by-sa/3.0/ or send a letter to Creative
// Commons, 171 Second Street, Suite 300, San Francisco, California 94105, USA.
//
// When attributing the artwork, using "GNOME Project" is enough.
// Please link to http://www.gnome.org where available.
const FALLBACK_ICON_PATH: &[u8] = include_bytes!("dialog-question-symbolic.svg");

pub fn fallback_icon() -> Svg {
    let handle = Handle::from_memory(FALLBACK_ICON_PATH);
    Svg::new(handle)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconPath {
    pub path: PathBuf,
    pub extension: Extension,
}

impl IconPath {
    pub fn lookup(name: &str, theme: &str, size: u16) -> Option<Self> {
        freedesktop_icons::lookup(name)
            .with_scale(1)
            .with_theme(theme)
            .with_size(size)
            .find()
            .and_then(Self::from_path)
    }

    pub fn absolute_from_icon_source(source: Option<&IconSource>) -> Option<Self> {
        source.and_then(|icon| {
            let path = match icon {
                IconSource::Name(icon) => icon,
                IconSource::Mime(icon) => icon,
            };

            IconPath::from_path(path.as_ref())
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

    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .map(|ext| ext.to_string_lossy())
            .and_then(|ext| Extension::try_from(ext.as_ref()).ok())
            .map(|extension| Self {
                path: path.as_ref().to_path_buf(),
                extension,
            })
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
