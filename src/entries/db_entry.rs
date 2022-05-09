use std::path::PathBuf;

use once_cell::sync::Lazy;
use pop_launcher::{self, IconSource};
use serde::Deserialize;

use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::run::RunCommandEntity;
use crate::db::web::WebEntity;
use crate::entries::AsEntry;
use crate::freedesktop::{Extension, IconPath};
use crate::ui::mode::WEB_CONFIG;

static TERMINAL_ICON: Lazy<Option<IconSource>> =
    Lazy::new(|| get_plugin_icon("terminal/plugin.ron"));
static WEB_ICON: Lazy<Option<IconSource>> = Lazy::new(|| get_plugin_icon("web/plugin.ron"));

#[derive(Deserialize)]
struct PluginConfig {
    icon: IconSource,
}

impl<'a> AsEntry<'a> for DesktopEntryEntity {
    fn get_display_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        self.icon.as_deref().and_then(IconPath::from_path)
    }
}

impl<'a> AsEntry<'a> for RunCommandEntity {
    fn get_display_name(&self) -> &str {
        self.command.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        IconPath::from_icon_source(TERMINAL_ICON.as_ref())
    }
}

impl<'a> AsEntry<'a> for WebEntity {
    fn get_display_name(&self) -> &str {
        self.query.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        WEB_CONFIG
            .as_ref()
            .and_then(|config| {
                config
                    .rules
                    .iter()
                    .find(|rule| rule.matches.contains(&self.kind))
            })
            // FIXME: see web/config.ron
            .map(|item| item.queries.first().unwrap().name.to_owned())
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
                    })
                } else if path.with_extension("ico").exists() {
                    ico_to_png(path.with_extension("ico"));
                    Some(IconPath {
                        path,
                        extension: Extension::Png,
                    })
                } else {
                    IconPath::from_icon_source(WEB_ICON.as_ref())
                };
            })
    }
}

fn get_plugin_icon(plugin: &str) -> Option<IconSource> {
    let path = pop_launcher::plugin_paths()
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
