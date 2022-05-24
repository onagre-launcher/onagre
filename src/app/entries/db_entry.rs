use iced::Row;
use std::borrow::Cow;
use std::path::PathBuf;

use crate::app::entries::AsEntry;
use crate::app::mode::WEB_CONFIG;
use crate::app::style::rows::RowStyles;
use crate::app::Message;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::icons::{Extension, IconPath};
use crate::THEME;

impl<'a> AsEntry<'a> for DesktopEntryEntity<'_> {
    fn get_display_name(&self) -> &str {
        self.name.as_ref()
    }

    fn get_icon(&self) -> Option<IconPath> {
        match &THEME.icon_theme {
            Some(theme) => self
                .icon
                .as_deref()
                .and_then(|name| IconPath::lookup(name, theme, THEME.icon_size)),
            _ => None,
        }
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        self.description.as_ref().cloned()
    }
}

impl<'a> AsEntry<'a> for PluginCommandEntity<'a> {
    // For plugin entities we use the category icon has the main icon
    fn get_icon_layout<'b>(
        &'a self,
        category_icon: Option<&'a IconPath>,
        style: &'static RowStyles,
    ) -> Row<'b, Message>
    where
        'b: 'a,
    {
        let icon = Self::build_icon(&style.icon, category_icon);
        Row::new().push(icon)
    }

    fn get_display_name(&self) -> &str {
        self.query.as_ref()
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl<'a> AsEntry<'a> for WebEntity<'a> {
    fn get_display_name(&self) -> &str {
        self.query.as_ref()
    }

    fn get_icon(&self) -> Option<IconPath> {
        WEB_CONFIG
            .get(&self.kind)
            .and_then(|definition| definition.first().map(|def| &def.name))
            .map(|web_query_kind| {
                (
                    dirs::cache_dir().unwrap().join("pop-launcher"),
                    web_query_kind,
                )
            })
            .and_then(|(path, filename)| {
                // Unfortunately we need to copy .ico files to png extension for iced
                // To render the icon
                let path = path.join(format!("{}.png", &filename));
                if path.exists() {
                    Some(IconPath {
                        path,
                        extension: Extension::Png,
                        symbolic: filename.ends_with("-symbolic"),
                    })
                } else if path.with_extension("ico").exists() {
                    ico_to_png(path.with_extension("ico"));
                    Some(IconPath {
                        path,
                        extension: Extension::Png,
                        symbolic: filename.ends_with("-symbolic"),
                    })
                } else {
                    None
                }
            })
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        None
    }
}

// FIXME: This should be removed
fn ico_to_png(path: PathBuf) {
    let file = std::fs::File::open(&path).unwrap();
    match ico::IconDir::read(file) {
        Ok(icon) => {
            for entry in icon.entries() {
                if !entry.is_png() {
                    let image = entry.decode().unwrap();
                    let file = std::fs::File::create(&path.with_extension("png")).unwrap();
                    image.write_png(file).unwrap();
                }
            }
        }
        Err(_) => {
            // We were unable to read the icon, it's probably a png
            std::fs::copy(&path, &path.with_extension("png")).unwrap();
        }
    }
}
