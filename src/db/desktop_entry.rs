use std::borrow::Cow;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::db::{Database, Entity};
use crate::freedesktop::desktop::DesktopEntry;

pub const COLLECTION: &str = "desktop-entries";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DesktopEntryEntity<'a> {
    pub name: Cow<'a, str>,
    pub icon: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub path: PathBuf,
    pub weight: u8,
}

impl<'a> Entity<'a> for DesktopEntryEntity<'a> {
    fn get_key(&self) -> Cow<'a, str> {
        self.name.clone()
    }
    fn get_weight(&self) -> u8 {
        self.weight
    }
}

impl<'a> DesktopEntryEntity<'a> {
    pub fn persist(entry: &'a DesktopEntry, path: &Path, db: &Database, theme: Option<&str>) {
        let weight = match db.get_by_key::<DesktopEntryEntity>(COLLECTION, &entry.name) {
            Some(de_entry) => de_entry.weight + 1,
            None => 0,
        };

        if let Some(theme) = theme {
            match entry.icon.as_ref() {
                Some(icon) => {
                    let icon = freedesktop_icons::lookup(icon)
                        .with_cache()
                        .with_theme(theme)
                        .find();
                    let icon = icon.unwrap().to_path_buf();
                    let icon = icon.to_str().map(Cow::Borrowed);
                    write_entity(db, entry, path, weight, icon)
                }
                None => {
                    let icon = entry.icon.as_deref().map(Cow::Borrowed);
                    write_entity(db, entry, path, weight, icon)
                }
            };
        } else {
            write_entity(db, entry, path, weight, None)
        }
    }
}

fn write_entity<'a>(
    db: &'a Database,
    entry: &'a DesktopEntry,
    path: &Path,
    weight: u8,
    icon: Option<Cow<'a, str>>,
) {
    let entity = DesktopEntryEntity {
        name: Cow::Borrowed(entry.name.as_ref()),
        icon,
        description: entry.comment.as_ref().cloned(),
        path: path.into(),
        weight,
    };

    db.insert(COLLECTION, &entity)
        .expect("Unable to insert history entry");
}
