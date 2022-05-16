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

impl Entity for DesktopEntryEntity<'_> {
    fn get_key(&self) -> Vec<u8> {
        self.name.as_bytes().to_vec()
    }
    fn get_weight(&self) -> u8 {
        self.weight
    }
}

impl<'a> DesktopEntryEntity<'a> {
    pub fn persist(entry: &'a DesktopEntry, path: &Path, db: &Database) {
        let weight = match db.get_by_key::<DesktopEntryEntity>(COLLECTION, &entry.name) {
            Some(de_entry) => de_entry.weight + 1,
            None => 0,
        };

        let entity = Self {
            name: Cow::Borrowed(entry.name.as_ref()),
            icon: entry.icon.as_deref().map(Cow::Borrowed),
            description: entry.comment.as_ref().map(|comment| comment.clone()),
            path: path.into(),
            weight,
        };

        db.insert(COLLECTION, &entity)
            .expect("Unable to insert history entry");
    }
}
