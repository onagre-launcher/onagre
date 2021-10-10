use pop_launcher;

use crate::db::desktop_entry::DesktopEntryEntity;
use crate::db::run::RunCommandEntity;
use crate::entries::AsEntry;
use crate::freedesktop::IconPath;
use pop_launcher::IconSource;
use serde::Deserialize;

lazy_static! {
    static ref TERMINAL_ICON: Option<IconSource> = get_plugin_icon();
}
#[derive(Deserialize)]
struct PluginConfig {
    icon: IconSource,
}

impl<'a> AsEntry<'a> for DesktopEntryEntity {
    fn get_display_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_icon(&self) -> Option<IconPath> {
        IconPath::from_path(&self.icon)
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

fn get_plugin_icon() -> Option<IconSource> {
    let path = pop_launcher::plugin_paths()
        .map(|path| path.as_ref().join("terminal/plugin.ron"))
        .find(|path| path.exists());

    path.map(std::fs::read_to_string)
        .map(Result::ok)
        .flatten()
        .map(|plugin| ron::from_str::<PluginConfig>(&plugin))
        .map(Result::ok)
        .flatten()
        .map(|plugin| plugin.icon)
}
