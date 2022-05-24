use std::borrow::Cow;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use pop_launcher_toolkit::launcher::IconSource;
use serde::Deserialize;

use crate::app::entries::AsEntry;
use crate::app::mode::WEB_CONFIG;
use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::plugin::PluginCommandEntity;
use crate::db::web::WebEntity;
use crate::icons::{Extension, IconPath};
use crate::THEME;

static TERMINAL_ICON: Lazy<Option<IconSource>> =
    Lazy::new(|| get_plugin_icon("terminal/plugin.ron"));

static WEB_ICON: Lazy<Option<IconSource>> = Lazy::new(|| get_plugin_icon("web/plugin.ron"));

#[derive(Deserialize)]
struct PluginConfig {
    icon: IconSource,
}

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

    // Should we persist that or can it be reconstructed from the desktop entry file
    fn get_category_icon(&self) -> Option<IconPath> {
        None
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        self.description.as_ref().cloned()
    }
}

impl<'a> AsEntry<'a> for PluginCommandEntity<'a> {
    fn get_display_name(&self) -> &str {
        self.query.as_ref()
    }

    // TODO: we should hold the plugin config here and get the icon from there
    fn get_icon(&self) -> Option<IconPath> {
        IconPath::absolute_from_icon_source(TERMINAL_ICON.as_ref())
    }

    // TODO: we should hold the plugin config here and get the category icon from there
    fn get_category_icon(&self) -> Option<IconPath> {
        None
    }

    // TODO: persist this
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
                return if path.exists() {
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
                    IconPath::absolute_from_icon_source(WEB_ICON.as_ref())
                };
            })
    }

    // TODO: we should hold the plugin config here and get the icon from there
    fn get_category_icon(&self) -> Option<IconPath> {
        None
    }

    fn get_description(&self) -> Option<Cow<'_, str>> {
        None
    }
}

fn get_plugin_icon(plugin: &str) -> Option<IconSource> {
    let path = pop_launcher_toolkit::launcher::plugin_paths()
        .map(|path| path.as_ref().join(plugin))
        .find(|path| path.exists());

    path.map(std::fs::read_to_string)
        .and_then(Result::ok)
        .map(|plugin| ron::from_str::<PluginConfig>(&plugin))
        .and_then(Result::ok)
        .map(|plugin| plugin.icon)
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
