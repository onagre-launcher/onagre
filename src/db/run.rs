use crate::db::{Database, Entity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunCommandEntity {
    pub command: String,
    pub weight: u8,
}

impl RunCommandEntity {
    pub fn persist(value: &str, db: &Database) {
        let command = db.get_by_key::<RunCommandEntity>(value);
        let weight = match command {
            None => 0,
            Some(command) => command.weight + 1,
        };

        let entity = RunCommandEntity {
            command: value.to_string(),
            weight,
        };

        db.insert(&entity)
            .expect("Unable to insert terminal cache entry");
    }
}

impl Entity for RunCommandEntity {
    fn get_key(&self) -> Vec<u8> {
        self.command.as_bytes().to_vec()
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }

    const COLLECTION: &'static str = "run_commands";
}
