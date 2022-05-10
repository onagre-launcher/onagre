use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::db::{Database, Entity};
use crate::freedesktop::desktop::DesktopEntry;

pub const COLLECTION: &'static str = "desktop-entries";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DesktopEntryEntity {
    pub name: String,
    pub icon: Option<String>,
    pub path: PathBuf,
    pub weight: u8,
}

impl Entity for DesktopEntryEntity {
    fn get_key(&self) -> Vec<u8> {
        self.name.as_bytes().to_vec()
    }
    fn get_weight(&self) -> u8 {
        self.weight
    }
}


impl DesktopEntryEntity {
    pub fn persist(entry: &DesktopEntry, path: &Path, db: &Database) {
        let weight = match db.get_by_key::<DesktopEntryEntity>(COLLECTION, &entry.name) {
            Some(de_entry) => de_entry.weight + 1,
            None => 0,
        };

        let entity = Self {
            name: entry.name.clone(),
            icon: entry.icon.clone(),
            path: path.into(),
            weight,
        };

        db.insert(COLLECTION, &entity).expect("Unable to insert history entry");
    }
}
