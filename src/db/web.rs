use crate::db::{Database, Entity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebEntity {
    pub query: String,
    pub kind: String,
    pub weight: u8,
}

impl WebEntity {
    pub fn persist(value: &str, kind: &str, db: &Database) {
        let command = db.get_by_key::<WebEntity>(value);
        let weight = match command {
            None => 0,
            Some(command) => command.weight + 1,
        };

        let entity = WebEntity {
            kind: kind.to_string(),
            query: value.to_string(),
            weight,
        };

        db.insert(&entity)
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

    const COLLECTION: &'static str = "web";
}
