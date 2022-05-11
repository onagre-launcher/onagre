use crate::db::{Database, Entity};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebEntity {
    pub query: String,
    pub kind: String,
    pub weight: u8,
}

impl WebEntity {
    pub fn persist(query: &str, kind: &str, db: &Database) {
        let command = db.get_by_key::<WebEntity>(kind, query);
        let weight = match command {
            None => 0,
            Some(command) => command.weight + 1,
        };

        let entity = WebEntity {
            kind: kind.to_string(),
            query: query.to_string(),
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

impl Entity for WebEntity {
    fn get_key(&self) -> Vec<u8> {
        self.query().as_bytes().to_vec()
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}
