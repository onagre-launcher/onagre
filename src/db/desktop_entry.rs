use std::borrow::Cow;
use std::path::{Path, PathBuf};

use crate::app::entries::Entry;
use crate::db::{icon_to_str, Database};
use crate::freedesktop::desktop::DesktopEntry;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub const DEFAULT_PLUGIN: &str = "desktop-entries";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DesktopEntryEntity<'a> {
    pub name: Cow<'a, str>,
    pub icon: Option<Cow<'a, str>>,
    pub category_icon: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub path: Option<PathBuf>,
    pub weight: u8,
}

impl<'a> DesktopEntryEntity<'a> {
    pub fn persist(entry: &dyn Entry, desktop_entry: &'a DesktopEntry, path: &Path, db: &Database) {
        let weight = match db.get_by_key(DEFAULT_PLUGIN, &desktop_entry.name) {
            Some(de_entry) => de_entry.weight + 1,
            None => 0,
        };

        let icon = entry.get_icon();
        let icon = icon_to_str(icon.as_ref());
        let category_icon = entry.get_category_icon();
        let category_icon = icon_to_str(category_icon.as_ref());

        let entity = DesktopEntryEntity {
            name: Cow::Borrowed(desktop_entry.name.as_ref()),
            icon,
            category_icon,
            description: desktop_entry.comment.as_ref().cloned(),
            path: Some(path.into()),
            weight,
        };

        db.insert(DEFAULT_PLUGIN, &entity)
            .expect("Unable to insert history entry");

        debug!("inserted entry: {:?}", entity);
    }

    pub fn persist_with_mode(entry: &dyn Entry, mode: &str, db: &Database) {
        let weight = match db.get_by_key(mode, entry.get_display_name()) {
            Some(de_entry) => de_entry.weight + 1,
            None => 0,
        };

        let icon = entry.get_icon();
        let icon = icon_to_str(icon.as_ref());
        let category_icon = entry.get_category_icon();
        let category_icon = icon_to_str(category_icon.as_ref());
        let description = entry.get_description();

        let entity = DesktopEntryEntity {
            name: Cow::Borrowed(entry.get_display_name()),
            icon,
            category_icon,
            description: description.as_deref().map(Cow::Borrowed),
            path: None,
            weight,
        };

        db.insert(DEFAULT_PLUGIN, &entity)
            .expect("Unable to insert history entry");

        debug!("inserted entry: {:?}", entity);
    }
}
