use crate::db::{Database, Entity};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginCommandEntity<'a> {
    pub(crate) query: Cow<'a, str>,
    pub weight: u8,
}

impl PluginCommandEntity<'_> {
    pub fn persist(collection: &str, query: &str, db: &Database) {
        let command = db.get_by_key::<PluginCommandEntity>(collection, query);
        let weight = match command {
            None => 0,
            Some(command) => command.weight + 1,
        };

        let entity = PluginCommandEntity {
            query: Cow::Borrowed(query),
            weight,
        };

        db.insert(collection, &entity)
            .expect("Unable to insert terminal cache entry");
    }
}

impl Entity for PluginCommandEntity<'_> {
    fn get_key(&self) -> Vec<u8> {
        self.query.as_bytes().to_vec()
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
