use crate::db::{Database, Entity};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tracing::debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebEntity<'a> {
    pub query: Cow<'a, str>,
    pub kind: Cow<'a, str>,
    pub weight: u8,
}

impl WebEntity<'_> {
    pub fn persist(query: &str, kind: &str, db: &Database) {
        let command = db.get_by_key::<WebEntity>(kind, query);
        let weight = match command {
            None => 0,
            Some(command) => command.weight + 1,
        };

        let entity = WebEntity {
            kind: Cow::Borrowed(kind),
            query: Cow::Borrowed(query),
            weight,
        };

        debug!("Inserting {entity:?} into '{kind}'");
        db.insert(&entity.kind, &entity)
            .expect("Unable to insert terminal cache entry");
    }

    pub fn query(&self) -> String {
        format!("{}{}", self.kind, self.query)
    }
}

impl Entity for WebEntity<'_> {
    fn get_key(&self) -> Vec<u8> {
        self.query().as_bytes().to_vec()
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
